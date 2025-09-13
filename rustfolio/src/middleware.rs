use axum::{
    extract::{FromRequestParts, State},
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::routes::auth::AuthUser;
use crate::state::AppState;

/// Middleware qui exige une session.
/// - Si ok: on met `AuthUser` dans `req.extensions()` puis on continue.
/// - Sinon: on redirige vers /auth/login.
pub async fn require_auth(
    State(st): State<AppState>,
    req: Request<axum::body::Body>,   // 👈 préciser le body
    next: Next,
) -> Result<Response, Response> {
    // On découpe la requête pour appeler l'extracteur
    let (mut parts, body) = req.into_parts();

    match AuthUser::from_request_parts(&mut parts, &st).await {
        Ok(user) => {
            // Reconstruire la requête et insérer l'utilisateur
            let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(user); // 👉 nécessite #[derive(Clone)] sur AuthUser
            Ok(next.run(req).await)
        }
        Err(_) => Err(Redirect::to("/auth/login").into_response()),
    }
}
