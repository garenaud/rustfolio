use axum::{
    routing::get,
    Router,
};
use axum::serve;
use tokio::net::TcpListener;
use std::net::SocketAddr;

async fn hello() -> &'static str {
    "Hello, Rust! 🚀"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        ;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener,app).await.unwrap();
}