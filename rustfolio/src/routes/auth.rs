use axum::{
    extract::{State, FromRequestParts, Query},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Redirect, Html},
    routing::{get, post},
    Form, Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use askama::Template;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};

use crate::state::AppState;
use lettre::message::{Mailbox, Message, SinglePart, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::extension::ClientId; // <= ici
use lettre::transport::smtp::response::Response;
use lettre::{SmtpTransport, Transport};

// ============================
// Templates
// ============================

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

// ============================
// Router
// ============================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login",  get(login_page).post(login_post))
        .route("/signup", get(signup_page).post(signup_post))
        .route("/logout", post(logout_post))
        .route("/verify", get(verify_email))
        .route("/me",     get(me))
}

// ============================
// Pages (GET)
// ============================

async fn login_page() -> impl IntoResponse {
    LoginTpl {
        year: chrono::Utc::now().year(),
        name: "Mon site",
        title: "Connexion",
        tagline: "Rust • Web • Cloud",
        error: None,
    }
    .render()
    .map(Html)
    .map_err(e500_html)
}

async fn signup_page() -> impl IntoResponse {
    SignupTpl {
        year: chrono::Utc::now().year(),
        name: "Mon site",
        title: "Créer un compte",
        tagline: "Rust • Web • Cloud",
        error: None,
    }
    .render()
    .map(Html)
    .map_err(e500_html)
}

// ============================
// POST /signup
// ============================

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
) -> Result<(CookieJar, Redirect), (StatusCode, Html<String>)> {
    // email déjà utilisé ?
    let exists: (i64,) = sqlx::query_as("SELECT COUNT(1) FROM users WHERE email = ?")
        .bind(&p.email)
        .fetch_one(&st.db)
        .await
        .map_err(e500_html)?;
    if exists.0 != 0 {
        let html = SignupTpl {
            year: chrono::Utc::now().year(),
            name: "Mon site",
            title: "Créer un compte",
            tagline: "Rust • Web • Cloud",
            error: Some("Email déjà utilisé"),
        }
        .render()
        .map(Html)
        .map_err(e500_html)?;
        return Err((StatusCode::CONFLICT, html));
    }

    // hash du mot de passe (Argon2)
    use rand::rngs::OsRng;
    use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2};
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(p.password.as_bytes(), &salt)
        .map_err(e500_html)?
        .to_string();

    let uid = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id,email,password_hash,display_name) VALUES (?,?,?,?)")
        .bind(&uid)
        .bind(&p.email)
        .bind(&hash)
        .bind(&p.display_name)
        .execute(&st.db)
        .await
        .map_err(e500_html)?;

    let verify_token = uuid::Uuid::new_v4().to_string();
    let verify_exp = (OffsetDateTime::now_utc() + Duration::hours(24)).format(&Rfc3339).unwrap();

    sqlx::query!(
        "INSERT INTO email_verifications (token, user_id, expires_at) VALUES (?,?,?)",
        verify_token,
        uid,
        verify_exp
    )
    .execute(&st.db)
    .await
    .map_err(e500_html)?;
    
    // 2) calcule l'URL publique (configurable)
    let base = std::env::var("PUBLIC_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".into());
    let verify_url = format!("{}/auth/verify?token={}", base, verify_token);

    send_verification_email(&p.email, &verify_url)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Erreur envoi mail: {e}"))))?;

    
    // 3) En DEV: log le lien (remplacera par un envoi d'email plus tard)
    eprintln!("DEV - lien de vérification pour {}: {}", p.email, verify_url);

    // crée session + cookie
    let (jar, _) = create_session_cookie(&st, jar, &uid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Erreur session: {e}"))))?;

    Ok((jar, Redirect::to("/")))
}

// ============================
// POST /login
// ============================

#[derive(Deserialize)]
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
            title: "Connexion",
            tagline: "Rust • Web • Cloud",
            error: Some("Identifiants invalides"),
        }
        .render()
        .map(Html)
        .map_err(e500_html)?;
        return Err((StatusCode::UNAUTHORIZED, html));
    };

    let pwd_hash = PasswordHash::new(&u.password_hash).map_err(e500_html)?;
    let ok = Argon2::default().verify_password(p.password.as_bytes(), &pwd_hash).is_ok();

    if !ok {
        let html = LoginTpl {
            year: chrono::Utc::now().year(),
            name: "Mon site",
            title: "Connexion",
            tagline: "Rust • Web • Cloud",
            error: Some("Identifiants invalides"),
        }
        .render()
        .map(Html)
        .map_err(e500_html)?;
        return Err((StatusCode::UNAUTHORIZED, html));
    }

    let (jar, _) = create_session_cookie(&st, jar, &u.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Erreur session: {e}"))))?;

    Ok((jar, Redirect::to("/")))
}

// ============================
// POST /logout
// ============================

async fn logout_post(State(st): State<AppState>, jar: CookieJar) -> (CookieJar, Redirect) {
    if let Some(sid) = jar.get("sid").map(|c| c.value().to_string()) {
        let _ = sqlx::query!("DELETE FROM sessions WHERE id = ?", sid)
            .execute(&st.db)
            .await;
    }
    (jar.remove(Cookie::from("sid")), Redirect::to("/auth/login"))
}

// ============================
// GET /verify?token=...
// ============================

#[derive(Deserialize)]
struct VerifyQuery { token: String }

async fn verify_email(
    State(st): State<AppState>,
    Query(q): Query<VerifyQuery>,
) -> Result<Redirect, (StatusCode, Html<String>)> {
    let rec = sqlx::query!(
        r#"
        SELECT user_id
        FROM email_verifications
        WHERE token = ?
          AND expires_at > CURRENT_TIMESTAMP
        "#,
        q.token
    )
    .fetch_optional(&st.db)
    .await
    .map_err(e500_html)?;

    let Some(rec) = rec else {
        let msg = Html("<h1>Lien invalide ou expiré</h1><p>Demande un nouvel e-mail de vérification.</p>".to_string());
        return Err((StatusCode::BAD_REQUEST, msg));
    };

    let mut tx = st.db.begin().await.map_err(e500_html)?;
    sqlx::query!("UPDATE users SET email_verified_at = CURRENT_TIMESTAMP WHERE id = ?", rec.user_id)
        .execute(&mut *tx)
        .await
        .map_err(e500_html)?;
    sqlx::query!("DELETE FROM email_verifications WHERE token = ?", q.token)
        .execute(&mut *tx)
        .await
        .map_err(e500_html)?;
    tx.commit().await.map_err(e500_html)?;

    Ok(Redirect::to("/?verified=1"))
}

// ===========================
// Send verification email
// ===========================

fn need(var: &str) -> anyhow::Result<String> {
    std::env::var(var).map_err(|_| anyhow::anyhow!("variable manquante: {}", var))
}

pub async fn send_verification_email(to: &str, verify_url: &str) -> anyhow::Result<()> {
    let mode = std::env::var("EMAIL_MODE").unwrap_or_else(|_| "log".into());
    let from = need("EMAIL_FROM")?;

    let subject = "Vérifie ton adresse email";
    let body_txt = format!(
        "Bienvenue !\n\nClique sur ce lien pour vérifier ton email :\n{}\n\nSi tu n'es pas à l'origine, ignore ce message.",
        verify_url
    );

    if mode == "log" {
        eprintln!("--- DEV EMAIL (no send) ---\nFROM: {}\nTO: {}\nSUBJECT: {}\n{}\n---------------------------",
                  from, to, subject, body_txt);
        return Ok(());
    }

    let smtp_host = need("SMTP_HOST")?;
    let smtp_port: u16 = need("SMTP_PORT")?.parse()?;
    let smtp_user = need("SMTP_USER")?;
    let smtp_pass = need("SMTP_PASS")?;

    let email = Message::builder()
        .from(from.parse::<Mailbox>()?)
        .to(to.parse::<Mailbox>()?)
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(body_txt),
        )?;

    let mailer = SmtpTransport::starttls_relay(&smtp_host)?
        .port(smtp_port)
        .credentials(Credentials::new(smtp_user, smtp_pass))
        .hello_name(ClientId::Domain("engagez-moi.com".to_string()))
        .build();

    let resp: Response = mailer.send(&email)?;
    let msg = resp.message().collect::<Vec<_>>().join(" ");
    eprintln!("SMTP response: {} {}", resp.code(), msg);

    Ok(())
}


// ============================
// GET /me (JSON protégé)
// ============================

#[derive(Serialize)]
struct Me { email: String, display_name: Option<String> }

async fn me(State(st): State<AppState>, user: AuthUser) -> Result<Json<Me>, (StatusCode, String)> {
    let u = sqlx::query!("SELECT email, display_name FROM users WHERE id = ?", user.id)
        .fetch_one(&st.db)
        .await
        .map_err(e500)?;
    Ok(Json(Me { email: u.email, display_name: u.display_name }))
}

// ============================
// Extracteur d'utilisateur
// ============================

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

// ============================
// Helpers
// ============================

use time::{OffsetDateTime, Duration, format_description::well_known::Rfc3339};

async fn create_session_cookie(
    st: &AppState,
    jar: CookieJar,
    user_id: &str,
) -> Result<(CookieJar, (StatusCode, &'static str)), sqlx::Error> {
    let sid = Uuid::new_v4().to_string();
    let exp = (OffsetDateTime::now_utc() + Duration::days(30)).format(&Rfc3339).unwrap();

    sqlx::query!("INSERT INTO sessions (id,user_id,expires_at) VALUES (?,?,?)", sid, user_id, exp)
        .execute(&st.db)
        .await?;

    // Active Secure via env en prod: COOKIE_SECURE=true
    let secure = std::env::var("COOKIE_SECURE").ok().as_deref() == Some("true");

    let cookie = Cookie::build(("sid", sid))
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(secure) // true en prod HTTPS
        .path("/")
        .max_age(time::Duration::days(30))
        .build();

    Ok((jar.add(cookie), (StatusCode::OK, "ok")))
}

// Erreurs 500 -> texte (pour JSON / extracteurs)
fn e500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {e}"))
}

// Erreurs 500 -> HTML (pour pages)
fn e500_html<E: std::fmt::Display>(e: E) -> (StatusCode, Html<String>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Internal error: {e}")))
}
