use axum::{
    Router,
    extract::State,
    http::{HeaderValue, Method},
    routing::get,
};
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod generate;
use generate::generate_terrain;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".into());

    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/generateterrain", get(generate_terrain))
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
