mod data;
mod state;
mod templates;
mod routes {
    pub mod pages;
    pub mod api;
    pub mod health;
    pub mod auth;
}

use axum::{routing::get, Router, serve};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use sqlx::SqlitePool;

use crate::state::AppState;
use crate::routes::{pages, api, health, auth};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = SqlitePool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL"))
        .await
        .expect("connect db");
    let exp: Vec<data::Experience> =
        serde_json::from_str(&std::fs::read_to_string("data/experience_fr.json").expect("read exp"))
            .expect("parse exp");
    let projects: Vec<data::Project> =
        serde_json::from_str(&std::fs::read_to_string("data/projects.json").expect("read projects"))
            .expect("parse projects");
    let skills: Vec<data::Skill> =
        serde_json::from_str(&std::fs::read_to_string("data/skills.json").expect("read skills"))
            .expect("parse skills");

    let state = AppState {
        db,
        _experiences: Arc::new(exp),
        projects: Arc::new(projects),
        skills: Arc::new(skills),
    };

    let app = Router::new()
        .route("/", get(pages::home))
        .route("/projects", get(pages::projects_page))
        .route("/portfolio", get(pages::portfolio_page))
        .route("/api/info", get(api::info_handler))
        .route("/api/projects", get(api::api_projects))
        .route("/health", get(health::health))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/data", ServeDir::new("data"))
        .nest("/auth", auth::router())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}