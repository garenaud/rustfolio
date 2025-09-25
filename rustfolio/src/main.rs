mod data;
mod state;
mod templates;
mod middleware;
mod types;
mod routes {
    pub mod pages;
    pub mod api;
    pub mod health;
    pub mod auth;
    pub mod profile;
    pub mod cv; 
    pub mod cv_normalized;
    pub mod skills;
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
    let _ = dotenvy::dotenv();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = SqlitePool::connect(&db_url)
        .await
        .expect("failed to connect DB");
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&db)
        .await
        .expect("enable FKs");

    let state = AppState {
        db,
        _experiences: Arc::new(Vec::<data::Experience>::new()),
        projects:     Arc::new(Vec::<data::Project>::new()),
        skills:       Arc::new(Vec::<data::Skill>::new()),
    };

    let assets_router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"));

    let dashboard_router = Router::new()
        .route("/dashboard", get(pages::dashboard_shell))
        .route("/dashboard/*rest", get(pages::dashboard_shell))
        .route_layer(from_fn_with_state(state.clone(), require_auth));

    let app = Router::new()
        .route("/", get(pages::home))
        .route("/api/info", get(api::info_handler))
        .route("/api/projects", get(api::api_projects))
        .route("/health", get(health::health))
        .nest("/auth", auth::router())
        .nest("/api", profile::router())
        .nest("/api", routes::cv::router())
        .nest("/api", routes::cv_normalized::router())
        .nest("/api", routes::skills::routes()) 
        .merge(assets_router)
        .merge(dashboard_router)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
