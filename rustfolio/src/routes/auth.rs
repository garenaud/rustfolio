use axum::{
    extract::{FromRequestParts, Query, State},
    http::{request::Parts, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use askama::Template;
//use askama_axum::IntoResponse as _; // permet aux Templates Askama de devenir une Response
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};

use crate::state::AppState;

use lettre::message::{header::ContentType, Mailbox, Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::extension::ClientId;
use lettre::transport::smtp::response::Response as SmtpResponse;
use lettre::{SmtpTransport, Transport};

// =====================================================
// Templates
// =====================================================

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

// =====================================================
// Router
// =====================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_page).post(login_post))
        .route("/signup", get(signup_page).post(signup_post))
        .route("/logout", post(logout_post))
        .route("/verify", get(verify_email))
        .route("/session", get(session))
        .route("/me", get(me))
}

// =====================================================
// GET pages
// =====================================================

async fn login_page() -> impl IntoResponse {
    LoginTpl {
        year: chrono::Utc::now().year(),
        name: "Mon site",
        title: "Connexion",
        tagline: "Rust • Web • Cloud",
        error: None,
    }
}

async fn signup_page() -> impl IntoResponse {
    SignupTpl {
        year: chrono::Utc::now().year(),
        name: "Mon site",
        title: "Créer un compte",
        tagline: "Rust • Web • Cloud",
        error: None,
    }
}

// =====================================================
// /auth/session -> { authenticated: bool }
// =====================================================

async fn session(State(st): State<AppState>, headers: HeaderMap) -> Json<serde_json::Value> {
    let jar = CookieJar::from_headers(&headers);
    let ok = if let Some(sid) = jar.get("sid").map(|c| c.value().to_string()) {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(1) FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP",
        )
        .bind(sid)
        .fetch_one(&st.db)
        .await
        .unwrap_or(0);
        count > 0
    } else {
        false
    };

    Json(serde_json::json!({ "authenticated": ok }))
}

// =====================================================
// POST /signup
// =====================================================

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
) -> Result<(CookieJar, Redirect), (StatusCode, Response)> {
    // email déjà utilisé ?
    let exists: (i64,) = sqlx::query_as("SELECT COUNT(1) FROM users WHERE email = ?")
        .bind(&p.email)
        .fetch_one(&st.db)
        .await
        .map_err(e500_resp)?;
    if exists.0 != 0 {
        return Err(with_status_tpl(
            StatusCode::CONFLICT,
            SignupTpl {
                year: chrono::Utc::now().year(),
                name: "Mon site",
                title: "Créer un compte",
                tagline: "Rust • Web • Cloud",
                error: Some("Email déjà utilisé"),
            },
        ));
    }

    // hash du mot de passe
    use argon2::password_hash::{PasswordHasher, SaltString};
    use rand::rngs::OsRng;

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(p.password.as_bytes(), &salt)
        .map_err(e500_resp)?
        .to_string();

    let uid = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id,email,password_hash,display_name) VALUES (?,?,?,?)")
        .bind(&uid)
        .bind(&p.email)
        .bind(&hash)
        .bind(&p.display_name)
        .execute(&st.db)
        .await
        .map_err(e500_resp)?;

    // token vérif email (24h)
    use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
    let verify_token = Uuid::new_v4().to_string();
    let verify_exp = (OffsetDateTime::now_utc() + Duration::hours(24))
        .format(&Rfc3339)
        .unwrap();

    sqlx::query!(
        "INSERT INTO email_verifications (token, user_id, expires_at) VALUES (?,?,?)",
        verify_token,
        uid,
        verify_exp
    )
    .execute(&st.db)
    .await
    .map_err(e500_resp)?;

    let base = std::env::var("PUBLIC_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".into());
    let verify_url = format!("{}/auth/verify?token={}", base, verify_token);

    // Envoi mail (ou log)
    send_verification_email(&p.email, &verify_url)
        .await
        .map_err(|e| e500_resp(format!("Erreur envoi mail: {e}")))?;

    // Session + cookie
    let (jar, _) = create_session_cookie(&st, jar, &uid)
        .await
        .map_err(|e| e500_resp(format!("Erreur session: {e}")))?;

    Ok((jar, Redirect::to("/")))
}

// =====================================================
// POST /login
// =====================================================

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

async fn login_post(
    State(st): State<AppState>,
    jar: CookieJar,
    Form(p): Form<LoginForm>,
) -> Result<(CookieJar, Redirect), (StatusCode, Response)> {
    let row = sqlx::query!(
        r#"
        SELECT id as "id!", password_hash
        FROM users
        WHERE email = ?
        "#,
        p.email
    )
    .fetch_optional(&st.db)
    .await
    .map_err(|e| e500_resp(format!("Erreur DB: {e}")))?;

    let Some(u) = row else {
        return Err(with_status_tpl(
            StatusCode::UNAUTHORIZED,
            LoginTpl {
                year: chrono::Utc::now().year(),
                name: "Mon site",
                title: "Connexion",
                tagline: "Rust • Web • Cloud",
                error: Some("Identifiants invalides"),
            },
        ));
    };

    let pwd_hash = PasswordHash::new(&u.password_hash).map_err(e500_resp)?;
    let ok = Argon2::default()
        .verify_password(p.password.as_bytes(), &pwd_hash)
        .is_ok();

    if !ok {
        return Err(with_status_tpl(
            StatusCode::UNAUTHORIZED,
            LoginTpl {
                year: chrono::Utc::now().year(),
                name: "Mon site",
                title: "Connexion",
                tagline: "Rust • Web • Cloud",
                error: Some("Identifiants invalides"),
            },
        ));
    }

    let (jar, _) = create_session_cookie(&st, jar, &u.id)
        .await
        .map_err(|e| e500_resp(format!("Erreur session: {e}")))?;

    Ok((jar, Redirect::to("/")))
}

// =====================================================
// POST /logout
// =====================================================

async fn logout_post(State(st): State<AppState>, jar: CookieJar) -> (CookieJar, Redirect) {
    if let Some(sid) = jar.get("sid").map(|c| c.value().to_string()) {
        let _ = sqlx::query!("DELETE FROM sessions WHERE id = ?", sid)
            .execute(&st.db)
            .await;
    }
    (jar.remove(Cookie::from("sid")), Redirect::to("/auth/login"))
}

// =====================================================
// GET /verify?token=...
// =====================================================

#[derive(Deserialize)]
struct VerifyQuery {
    token: String,
}

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
        let msg = Html("<h1>Lien invalide ou expiré</h1><p>Demande un nouvel e-mail.</p>".to_string());
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

// =====================================================
// GET /me
// =====================================================

#[derive(Serialize)]
struct Me {
    email: String,
    display_name: Option<String>,
}

async fn me(State(st): State<AppState>, user: AuthUser) -> Result<Json<Me>, (StatusCode, String)> {
    let u = sqlx::query!("SELECT email, display_name FROM users WHERE id = ?", user.id)
        .fetch_one(&st.db)
        .await
        .map_err(e500)?;
    Ok(Json(Me {
        email: u.email,
        display_name: u.display_name,
    }))
}

// =====================================================
// Extracteur d'utilisateur
// =====================================================

#[derive(Clone)]
pub struct AuthUser {
    pub id: String,
}

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let Some(sid) = jar.get("sid").map(|c| c.value().to_string()) else {
            return Err((StatusCode::UNAUTHORIZED, "Non connecté".into()));
        };

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

// =====================================================
// Helpers
// =====================================================

use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

async fn create_session_cookie(
    st: &AppState,
    jar: CookieJar,
    user_id: &str,
) -> Result<(CookieJar, (StatusCode, &'static str)), sqlx::Error> {
    let sid = Uuid::new_v4().to_string();
    let exp = (OffsetDateTime::now_utc() + Duration::days(30))
        .format(&Rfc3339)
        .unwrap();

    sqlx::query!("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)", sid, user_id, exp)
        .execute(&st.db)
        .await?;

    let secure = std::env::var("COOKIE_SECURE").ok().as_deref() == Some("true");

    let cookie = Cookie::build(("sid", sid))
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(secure)
        .path("/")
        .max_age(time::Duration::days(30))
        .build();

    Ok((jar.add(cookie), (StatusCode::OK, "ok")))
}

// Erreurs 500 -> texte
fn e500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {e}"))
}

// Erreurs 500 -> Html
fn e500_html<E: std::fmt::Display>(e: E) -> (StatusCode, Html<String>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Internal error: {e}")))
}

// Erreurs 500 -> Response
fn e500_resp<E: std::fmt::Display>(e: E) -> (StatusCode, Response) {
    (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("Internal error: {e}")).into_response())
}

// Helpers pour status + template
fn with_status_tpl<T>(code: StatusCode, tpl: T) -> (StatusCode, Response)
where
    T: Template + askama_axum::IntoResponse,
{
    (code, tpl.into_response())
}

// =====================================================
// Email
// =====================================================

fn need(var: &str) -> anyhow::Result<String> {
    std::env::var(var).map_err(|_| anyhow::anyhow!("variable manquante: {}", var))
}

pub async fn send_verification_email(to: &str, verify_url: &str) -> anyhow::Result<()> {
    let mode = std::env::var("EMAIL_MODE").unwrap_or_else(|_| "log".into());
    let from = need("EMAIL_FROM")?;

    let subject = "Vérifie ton adresse email";
    let body_txt = format!(
        "Bienvenue !\n\nClique sur ce lien pour vérifier ton email :\n{}\n\n",
        verify_url
    );

    if mode == "log" {
        eprintln!(
            "--- DEV EMAIL (no send) ---\nFROM: {}\nTO: {}\nSUBJECT: {}\n{}\n---------------------------",
            from, to, subject, body_txt
        );
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

    let resp: SmtpResponse = mailer.send(&email)?;
    let msg = resp.message().collect::<Vec<_>>().join(" ");
    eprintln!("SMTP response: {} {}", resp.code(), msg);

    Ok(())
}
