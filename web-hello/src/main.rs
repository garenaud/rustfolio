use axum::{
    routing::{get, post},
    extract::{Path, Query},
    response::{IntoResponse, Response},
    Json, Router, serve,
};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use std::collections::HashMap;
use axum::http::StatusCode;

#[derive(serde::Deserialize)]
struct RegisterInput {
    name: String,
    age: u8,
}

#[derive(serde::Serialize)]
struct RegisterOk {
    id: u64,
    name: String,
    age: u8,
}

#[derive(serde::Serialize)]
struct ErrorMsg {
    error: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Info {
    status: &'static str,
    app: &'static str,
    version: &'static str,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Person {
    name: String,
    age: u8,
}

async fn echo(Json(payload): Json<Person>) -> Json<Person> {
    Json(payload)
}

async fn info_handler() -> Json<Info> {
    Json(Info {
        status: "ok",
        app: "web-hello",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn register(Json(payload): Json<RegisterInput>) -> Response {
    if payload.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ErrorMsg {
            error: "name is required".into(),
        })).into_response();
    }
    if payload.age == 0 {
        return (StatusCode::BAD_REQUEST, Json(ErrorMsg {
            error: "age must be > 0".into(),
        })).into_response();
    }

    let created = RegisterOk { id: 1, name: payload.name, age: payload.age };
    (StatusCode::CREATED, Json(created)).into_response()
}

async fn greet(Query(params): Query<HashMap<String, String>>) -> String {
    if let Some(name) = params.get("name") {
        format!("Hello, {}!", name)
    } else {
        "Hello, stranger!".to_string()
    }
}

async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}! 🚀 ca fonctionne", name)
}

async fn hello() -> &'static str {
    "Hello, Rust! 🚀 ca fonctionne"
}

async fn health() -> &'static str {
    "OK c'est bon"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/api/info", get(info_handler))
        .route("/hello/:name", get(hello_name))
        .route("/greet", get(greet))
        .route("/api/echo", axum::routing::post(echo))
        .route("/api/register", post(register));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
