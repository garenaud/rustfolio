use axum::{
    body::Body,
    http::{Request, StatusCode},
    extract::State,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use crate::state::AppState;
use sqlx::Row;

pub async fn require_auth<B>(
    State(app): axum::extract::State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
    let jar = CookieJar::from_headers(req.headers());
    let Some(sid) = jar.get("sid").map(|c| c.value().to_string()) else {
        return Err((StatusCode::UNAUTHORIZED, "Non connecté".into()));
    };

    let ok = sqlx::query("SELECT 1 FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP")
        .bind(&sid)
        .fetch_optional(&app.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}")))?
        .is_some();

    if !ok {
        return Err((StatusCode::UNAUTHORIZED, "Session expirée".into()));
    }

    Ok(next.run(req).await)
}
