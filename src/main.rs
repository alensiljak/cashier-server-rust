use axum::{
    routing::{get,post},
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    //println!("Hello, world!");

    // tracing init
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "cashier-server=debug".into()),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new().allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        //.route("/", get(|| async { "Hello, World!" }))
        .route("/", get(root))
        .route("/ledger", get(ledger))
        .route("/ping", get(|| async { "pong" }))
        .layer(cors);

    let address = SocketAddr::from(([0,0,0,0], 3000));
    tracing::debug!("listening on {}", address);

    // run it with hyper on localhost:3000
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn ledger(
    // this argument tells axum to parse the request body
    // as JSON into a `LedgerCommand` type
    Json(payload): Json<LedgerCommand>
) -> String {
    //todo: run ledger

    return format!("Ledger endpoint. You asked for {}", payload.command);
}

#[derive(Deserialize)]
struct LedgerCommand {
    command: String,
}

#[derive(Serialize)]
struct LedgerOutput {
    output: String,
}
