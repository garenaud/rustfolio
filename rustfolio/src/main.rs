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
    response::IntoResponse,               // <-- nécessaire pour .into_response()
    routing::{get, get_service},
    serve, Router,
};
use axum::http::StatusCode;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

use crate::middleware::require_auth;
use crate::routes::{api, auth, health, pages, profile};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Charge les variables du .env (dev)
    dotenvy::dotenv().ok();

    // --- DB ---
    let db = SqlitePool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("failed to connect DB");

    // --- Données JSON statiques (si tu les utilises dans tes pages SSR) ---
    let exp: Vec<data::Experience> = serde_json::from_str(
        &std::fs::read_to_string("data/experience_fr.json").expect("read experience_fr.json"),
    )
    .expect("parse experience_fr.json");

    let projects: Vec<data::Project> = serde_json::from_str(
        &std::fs::read_to_string("data/projects.json").expect("read projects.json"),
    )
    .expect("parse projects.json");

    let skills: Vec<data::Skill> = serde_json::from_str(
        &std::fs::read_to_string("data/skills.json").expect("read skills.json"),
    )
    .expect("parse skills.json");

    // --- State partagé ---
    let state = AppState {
        db,
        _experiences: Arc::new(exp),
        projects: Arc::new(projects),
        skills: Arc::new(skills),
    };

    // --- SPA tableau de bord protégée (/dashboard) ---
    // Sert les fichiers statiques et fallback sur index.html pour /dashboard/*.
    let dashboard_service = get_service(
        ServeDir::new("assets/dashboard")
            .fallback(ServeFile::new("assets/dashboard/index.html")),
    )
    .handle_error(|err| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("static error: {err}"),
        )
    });

    let dashboard_router = Router::new()
        .nest_service("/dashboard", dashboard_service)
        .route_layer(from_fn_with_state(state.clone(), require_auth));

    // --- App principale ---
    let app = Router::new()
        // Pages SSR
        .route("/", axum::routing::get(pages::home))
        // .route("/projects", get(pages::projects_page))
        // .route("/portfolio", get(pages::portfolio_page))

        // API publiques
        .route("/api/info", get(api::info_handler))
        .route("/api/projects", get(api::api_projects))

        // Santé & statiques (hors dashboard)
        .route("/health", get(health::health))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/data", ServeDir::new("data"))

        // Auth & API profil
        .nest("/auth", auth::router())
        .nest("/api", profile::router())

        // Debug Askama : rend directement le template HomeTpl (idéal pour diagnostiquer)
        .route("/__debug/inline", axum::routing::get(pages::debug_inline))

        .route("/__debug/home_raw", get(|| async {
            use askama::Template;
            use crate::templates::HomeTpl;
            use chrono::Datelike;

            let t = HomeTpl { year: chrono::Utc::now().year() };
            match t.render() {
                Ok(html) => axum::response::Html(html).into_response(),
                Err(e) => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Askama render error: {e:?}")
                ).into_response(),
            }
        }))

        // Dashboard (protégé)
        .merge(dashboard_router)

        // State pour tout l’arbre
        .with_state(state);

    // --- Serveur HTTP (pas HTTPS en dev) ---
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
