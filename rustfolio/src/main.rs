mod data;
mod state;
mod templates;
mod middleware;
mod routes {
    pub mod pages;
    pub mod api;
    pub mod health;
    pub mod auth;
    pub mod profile;
}

use std::{net::SocketAddr, sync::Arc};

use axum::{
    middleware::from_fn_with_state,
    routing::get,
    serve, Router,
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::middleware::require_auth;
use crate::routes::{api, auth, health, pages, profile};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    // charge .env en dev (ne panique pas si absent)
    let _ = dotenvy::dotenv();

    // --- DB ---
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = SqlitePool::connect(&db_url)
        .await
        .expect("failed to connect DB");

    // --- State partagé ---
    let state = AppState {
        db,
        _experiences: Arc::new(Vec::<data::Experience>::new()),
        projects:     Arc::new(Vec::<data::Project>::new()),
        skills:       Arc::new(Vec::<data::Skill>::new()),
    };

    // --- Assets statiques globaux ---
    // Sert /assets depuis le dossier local "assets"
    // et applique automatiquement le bon Content-Type (dont application/wasm)
    // + support des fichiers précompressés .br/.gz
    let assets_router = Router::new()
        .nest_service(
            "/assets",
            ServeDir::new("assets")
                .precompressed_br()
                .precompressed_gzip(),
        );

    // --- Dashboard protégé (shell SSR qui charge la SPA Yew) ---
    let dashboard_router = Router::new()
        .route("/dashboard", get(pages::dashboard_shell))
        .route("/dashboard/*rest", get(pages::dashboard_shell))
        .route_layer(from_fn_with_state(state.clone(), require_auth));

    // --- App principale ---
    let app = Router::new()
        .route("/", get(pages::home))
        .route("/api/info", get(api::info_handler))
        .route("/api/projects", get(api::api_projects))
        .route("/health", get(health::health))
        .nest("/auth", auth::router())
        .nest("/api", profile::router())
        .merge(assets_router)
        .merge(dashboard_router)
        .with_state(state);

    // --- Serveur HTTP ---
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
