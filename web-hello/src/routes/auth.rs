use axum::{
    extract::{State, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Redirect, Html},
    routing::{get, post},
    Form, Json, Router,
};
use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

// ---------- Templates pages ----------
use askama::Template;
use chrono::Datelike;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTpl<'a> {
    year: i32,
    name: &'a str,
    title: &'a str,
    tagline: &'a str,
    error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "auth/signup.html")]
struct SignupTpl<'a> {
    year: i32,
    name: &'a str,
    title: &'a str,
    tagline: &'a str,
    error: Option<&'a str>,
}

// ---------- Sous-routeur ----------
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login",  get(login_page).post(login_post))
        .route("/signup", get(signup_page).post(signup_post))
        .route("/logout", post(logout_post))
        .route("/me",     get(me))
}

// ---------- Pages ----------
async fn login_page() -> impl IntoResponse {
    LoginTpl {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        error: None,
    }
    .render()
    .map(axum::response::Html)
    .map_err(e500)
}

async fn signup_page() -> impl IntoResponse {
    SignupTpl {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        error: None,
    }
    .render()
    .map(axum::response::Html)
    .map_err(e500)
}

// ---------- POST /signup ----------
#[derive(Deserialize)]
struct SignupForm {
    email: String,
    password: String,
    display_name: Option<String>,
}

async fn signup_post(
    State(st): State<AppState>,
    jar: CookieJar,
    Form(p): Form<SignupForm>,
) -> Result<(CookieJar, Redirect), (StatusCode, String)> {
    let exists: (i64,) = sqlx::query_as("SELECT COUNT(1) FROM users WHERE email = ?")
        .bind(&p.email)
        .fetch_one(&st.db)
        .await
        .map_err(e500)?;
    if exists.0 != 0 {
        let html = SignupTpl {
            year: chrono::Utc::now().year(),
            name: "Gaëtan Renaud",
            title: "Développeur Rust",
            tagline: "Rust • Web • Cloud",
            error: Some("Email déjà utilisé"),
        }
        .render()
        .map(axum::response::Html)
        .map_err(e500)?;
        return Err((StatusCode::CONFLICT, html.0));
    }

    // hash
    use rand::rngs::OsRng;
    use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2};
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(p.password.as_bytes(), &salt)
        .map_err(e500)?
        .to_string();

    let uid = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id,email,password_hash,display_name) VALUES (?,?,?,?)")
        .bind(&uid)
        .bind(&p.email)
        .bind(&hash)
        .bind(&p.display_name)
        .execute(&st.db)
        .await
        .map_err(e500)?;

    let (jar, _) = create_session_cookie(&st, jar, &uid).await.map_err(e500)?;
    Ok((jar, Redirect::to("/")))
}

// ---------- POST /login ----------
#[derive(serde::Deserialize)]
struct LoginForm { email: String, password: String }

async fn login_post(
    State(st): State<AppState>,
    jar: CookieJar,
    Form(p): Form<LoginForm>,
) -> Result<(CookieJar, Redirect), (StatusCode, Html<String>)> {
    let row = sqlx::query!(
        r#"
        SELECT
          id as "id!",
          password_hash
        FROM users
        WHERE email = ?
        "#,
        p.email
    )
    .fetch_optional(&st.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Erreur DB: {e}"))))?;

    let Some(u) = row else {
        let html = LoginTpl {
            year: chrono::Utc::now().year(),
            name: "Mon site",
            title: "Bienvenue",
            tagline: "Rust • Web • Cloud",
            error: Some("Identifiants invalides"),
        }
        .render()
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Erreur template: {e}"))))?;
        return Err((StatusCode::UNAUTHORIZED, html));
    };

    let pwd_hash = PasswordHash::new(&u.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Hash invalide: {e}"))))?;

    let ok = Argon2::default().verify_password(p.password.as_bytes(), &pwd_hash).is_ok();

    if !ok {
        let html = LoginTpl {
            year: chrono::Utc::now().year(),
            name: "Mon site",
            title: "Bienvenue",
            tagline: "Rust • Web • Cloud",
            error: Some("Identifiants invalides"),
        }
        .render()
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Erreur template: {e}"))))?;
        return Err((StatusCode::UNAUTHORIZED, html));
    }

    let (jar, _) = create_session_cookie(&st, jar, &u.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Erreur session: {e}"))))?;

    Ok((jar, Redirect::to("/")))
}


// ---------- POST /logout ----------
async fn logout_post(State(st): State<AppState>, jar: CookieJar) -> (CookieJar, Redirect) {
    if let Some(sid) = jar.get("sid").map(|c| c.value().to_string()) {
        let _ = sqlx::query!("DELETE FROM sessions WHERE id = ?", sid).execute(&st.db).await;
    }
    (jar.remove(Cookie::from("sid")), Redirect::to("/auth/login"))
}


// ---------- GET /me (JSON)
#[derive(Serialize)]
struct Me { email: String, display_name: Option<String> }

async fn me(State(st): State<AppState>, user: AuthUser) -> Result<Json<Me>, (StatusCode, String)> {
    let u = sqlx::query!("SELECT email, display_name FROM users WHERE id = ?", user.id)
        .fetch_one(&st.db)
        .await
        .map_err(e500)?;
    Ok(Json(Me { email: u.email, display_name: u.display_name }))
}

// ---------- Extracteur d'utilisateur ----------
pub struct AuthUser { pub id: String }

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let Some(sid) = jar.get("sid").map(|c| c.value().to_string())
            else { return Err((StatusCode::UNAUTHORIZED, "Non connecté".into())); };

        let rec = sqlx::query!(
            "SELECT user_id FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP",
            sid
        )
        .fetch_optional(&state.db)
        .await
        .map_err(e500)?;

        rec.map(|r| AuthUser { id: r.user_id })
           .ok_or((StatusCode::UNAUTHORIZED, "Session expirée".into()))
    }
}

// ---------- Helpers ----------
use time::{OffsetDateTime, Duration, format_description::well_known::Rfc3339};

async fn create_session_cookie(
    st: &AppState,
    jar: CookieJar,
    user_id: &str,
) -> Result<(CookieJar, (StatusCode, &'static str)), sqlx::Error> {
    let sid = Uuid::new_v4().to_string();
    let exp = (OffsetDateTime::now_utc() + Duration::days(30))
        .format(&Rfc3339).unwrap();

    sqlx::query!("INSERT INTO sessions (id,user_id,expires_at) VALUES (?,?,?)", sid, user_id, exp)
        .execute(&st.db)
        .await?;

    let cookie = Cookie::build(("sid", sid))
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false) // true en prod HTTPS
        .path("/")
        .max_age(time::Duration::days(30))
        .build();

    Ok((jar.add(cookie), (StatusCode::OK, "ok")))
}

fn e500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {e}"))
}
