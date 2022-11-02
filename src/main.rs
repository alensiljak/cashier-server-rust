use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    // Json,
    Router,
};
//use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, process::Command};
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

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
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
    let query = params["command"].as_str();

    let ledger_output = run_ledger(query);

    let result = format!(
        "Ledger response. You asked for: {}. Ledger replied: {}",
        query, ledger_output
    );
    (StatusCode::OK, result)
}

fn run_ledger(command: &str) -> String {
    // separate command into individual arguments
    let iter = command.split_whitespace();

    let mut ledger = Command::new("ledger");
    ledger.args(iter);

    //let output = ledger.status().expect("process failed to execute");
    let output = ledger.output().expect("failed to execute process");
    //let output = ledger.spawn().expect("ls command failed to start");

    // assert!(output.status.success());
    let result: String;

    if !output.status.success() {
        result = String::from_utf8(output.stderr).unwrap();
        // println!("not success: {}", result);
    } else {
        result = String::from_utf8(output.stdout).unwrap();
    }

    return result;
}
