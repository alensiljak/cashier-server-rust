use axum::{
    extract::Query,
    routing::{get,post},
    response::IntoResponse,
    Json, Router, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::field::debug;
use std::{net::SocketAddr, collections::HashMap, future::Future};
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
        .route("/", get(ledger))
        .route("/hello", get(hello_img))
        //.route("/ledger", get(ledger))
        .route("/ping", get(|| async { "pong" }))
        .route("/test", get(root))
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
pub async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello_img() -> String {
    // todo: decode pixel

    let result = "".to_string();
    return result;
}

async fn ledger(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // impl Future <Output = impl IntoResponse>
    //todo: run ledger

    //return format!("Ledger endpoint. You asked for {}", payload);
    //let parameters: Vec<String> = params.into_keys().collect();
    //let values = params.values();
    let query = params["command"].as_str();

    //tracing::debug!(parameters);

    let result = format!("Ledger endpoint. You asked for {}", query);
    (StatusCode::OK, result)
}

// #[derive(Deserialize)]
// struct LedgerCommand {
//     command: String,
// }

// #[derive(Serialize)]
// struct LedgerOutput {
//     output: String,
// }
