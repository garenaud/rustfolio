mod data;

use axum::{
    routing::{get, post},
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    Json, Router, serve,
};
use axum::http::StatusCode;
use tokio::net::TcpListener;
use std::{net::SocketAddr, collections::HashMap, fs, sync::Arc};
use askama::Template;
use askama_axum::IntoResponse as _;
use tower_http::services::ServeDir;
use chrono::Datelike;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    year: i32,
    name: &'a str,
    title: &'a str,
    tagline: &'a str,
    skills: &'a [data::Skill],
    projects: &'a [data::Project],
}

async fn home(State(st): State<AppState>) -> impl IntoResponse {
    let tpl = IndexTemplate {
        year: chrono::Utc::now().year(),
        name: "Gaëtan Renaud",
        title: "Développeur Rust",
        tagline: "Rust • Web • Cloud",
        skills: &st.skills[..std::cmp::min(5, st.skills.len())],
        projects: &st.projects[..std::cmp::min(3, st.projects.len())],
    };
    tpl.into_response()
}

#[derive(Clone)]
struct AppState {
    experiences: Arc<Vec<data::Experience>>,
    projects: Arc<Vec<data::Project>>,
    skills: Arc<Vec<data::Skill>>,
}

#[derive(serde::Deserialize)]
struct RegisterInput { name: String, age: u8 }

#[derive(serde::Serialize)]
struct RegisterOk { id: u64, name: String, age: u8 }

#[derive(serde::Serialize)]
struct ErrorMsg { error: String }

#[derive(serde::Serialize, serde::Deserialize)]
struct Info { status: &'static str, app: &'static str, version: &'static str }

#[derive(serde::Serialize, serde::Deserialize)]
struct Person { name: String, age: u8 }

async fn echo(Json(payload): Json<Person>) -> Json<Person> { Json(payload) }

async fn info_handler() -> Json<Info> {
    Json(Info { status: "ok", app: "web-hello", version: env!("CARGO_PKG_VERSION") })
}

async fn register(Json(payload): Json<RegisterInput>) -> Response {
    if payload.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ErrorMsg { error: "name is required".into() })).into_response();
    }
    if payload.age == 0 {
        return (StatusCode::BAD_REQUEST, Json(ErrorMsg { error: "age must be > 0".into() })).into_response();
    }
    let created = RegisterOk { id: 1, name: payload.name, age: payload.age };
    (StatusCode::CREATED, Json(created)).into_response()
}

async fn greet(Query(params): Query<HashMap<String, String>>) -> String {
    if let Some(name) = params.get("name") { format!("Hello, {}!", name) } else { "Hello, stranger!".to_string() }
}

async fn hello_name(Path(name): Path<String>) -> String { format!("Hello, {}! 🚀 ca fonctionne", name) }
async fn hello() -> &'static str { "Hello, Rust! 🚀 ca fonctionne" }
async fn health() -> &'static str { "OK c'est bon" }

#[tokio::main]
async fn main() {
    let exp: Vec<data::Experience> =
        serde_json::from_str(&fs::read_to_string("data/experience_fr.json").expect("read exp"))
        .expect("parse exp");
    let projects: Vec<data::Project> =
        serde_json::from_str(&fs::read_to_string("data/projects.json").expect("read projects"))
        .expect("parse projects");
    let skills: Vec<data::Skill> =
        serde_json::from_str(&fs::read_to_string("data/skills.json").expect("read skills"))
        .expect("parse skills");

    let state = AppState {
        experiences: Arc::new(exp),
        projects: Arc::new(projects),
        skills: Arc::new(skills),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/api/info", get(info_handler))
        .route("/hello/:name", get(hello_name))
        .route("/greet", get(greet))
        .route("/api/echo", post(echo))
        .route("/api/register", post(register))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
