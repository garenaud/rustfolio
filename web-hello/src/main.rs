use axum::{
    routing::get,
    extract::{Path, Query},
    Json, Router, serve,
};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use std::collections::HashMap;

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
        .route("/api/echo", axum::routing::post(echo));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
