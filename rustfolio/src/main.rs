use axum::{middleware::from_fn_with_state, routing::get, serve, Router};
use axum::http::Method;
use http::HeaderValue;
use sqlx::SqlitePool;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tower_http::cors::Any;

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
use crate::middleware::require_auth;
use crate::routes::{api, auth, health, pages, profile};
use crate::routes::{cv_normalized, skills};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    // DB
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = SqlitePool::connect(&db_url).await.expect("failed to connect DB");
    sqlx::query("PRAGMA foreign_keys = ON;").execute(&db).await.expect("enable FKs");

    let state = AppState {
        db,
        _experiences: Arc::new(Vec::<data::Experience>::new()),
        projects: Arc::new(Vec::<data::Project>::new()),
        skills: Arc::new(Vec::<data::Skill>::new()),
    };

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:8081"),
            HeaderValue::from_static("http://127.0.0.1:8081"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS, Method::PATCH])
        .allow_headers(Any); 

    let assets_router = Router::new().nest_service(
        "/assets",
        ServeDir::new("assets").precompressed_br().precompressed_gzip(),
    );

    let dashboard_router = Router::new()
        .route("/dashboard", get(pages::dashboard_shell))
        .route("/dashboard/*rest", get(pages::dashboard_shell))
        .route_layer(from_fn_with_state(state.clone(), require_auth));

    let api_router = Router::new()
        .route("/info", get(api::info_handler))
        .route("/projects", get(api::api_projects))
        .merge(profile::router())
        .merge(routes::cv::router())
        .merge(cv_normalized::router())
        .merge(skills::routes());

    let app = Router::new()
        .route("/", get(pages::home))
        .route("/health", get(health::health))
        .nest("/auth", auth::router())
        .nest("/api", api_router)
        .merge(assets_router)
        .merge(dashboard_router)
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
