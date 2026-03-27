/*!
 * CLI for Cashier Server
 */

use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use axum::response::AppendHeaders;
use base64::{engine::general_purpose, Engine};
use serde::Serialize;
use std::collections::HashMap;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
extern crate base64;
use tokio::process::Command;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    initialize_logging();

    let cors = CorsLayer::new().allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(ledger))
        .route("/hello", get(hello_img))
        .route("/ping", get(|| async { "pong" }))
        // .route("/reload", get(reload))
        .route("/infrastructure/config", get(get_config))
        .route("/infrastructure/accounts", get(get_accounts))
        .route("/infrastructure/commodities", get(get_commodities))
        .route("/shutdown", get(shutdown))
        // middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    log::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

/**
 * Initialize and configure logging/tracing to the console window.
 */
fn initialize_logging() {
    let formatting_layer = tracing_subscriber::fmt::layer();

    let filter = filter::Targets::new()
        .with_target("cashier_server", Level::TRACE)
        .with_target("tower_http::trace::on_response", Level::DEBUG)
        // .with_target("tower_http::trace::on_request", Level::DEBUG)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(formatting_layer)
        .with(filter)
        .init();
}

async fn hello_img() -> impl IntoResponse {
    // Base64 encoded pixel
    let pixel_encoded = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
    let decoded = general_purpose::STANDARD.decode(pixel_encoded);

    (
        AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        decoded.unwrap(),
    )
}

async fn ledger(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    log::debug!("ledger: {:?}", params);

    if !params.contains_key("command") {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("No Ledger command sent"));
        return (StatusCode::BAD_REQUEST, Json(result));
    }

    let query = params["command"].as_str();

    let ledger_output = run_ledger(query).await;

    // split lines
    //let rows: Vec<String> = ledger_output.lines().collect();
    let rows: Vec<String> = ledger_output.lines().map(|x| String::from(x)).collect();

    // convert to Json
    (StatusCode::OK, Json(rows))
}

// Reload is only use with Beancount in-memory cache.
// We don't have this when calling a CLI engine.
// async fn reload() -> impl IntoResponse {
//     log::info!("Reloading Beancount data");

//     // Refresh environment variables from .env file
//     dotenv().ok();

//     if let Ok(bean_file) = std::env::var("BEANCOUNT_FILE") {
//         log::info!("Loading Beancount file: {}", bean_file);
//         // Future implementation: Re-initialize in-memory connection or cache here
//     }

//     (StatusCode::OK, "Reloaded")
// }

#[derive(Serialize)]
struct InfrastructureResponse {
    content: String,
}

async fn get_config() -> impl IntoResponse {
    read_infrastructure_file("config.bean").await
}

async fn get_accounts() -> impl IntoResponse {
    read_infrastructure_file("accounts.bean").await
}

async fn get_commodities() -> impl IntoResponse {
    read_infrastructure_file("commodities.bean").await
}

/**
 * Shared logic to read Beancount infrastructure files relative to the main ledger file.
 */
async fn read_infrastructure_file(filename: &str) -> impl IntoResponse {
    let bean_file = match std::env::var("BEANCOUNT_FILE") {
        Ok(v) => v,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "BEANCOUNT_FILE environment variable not set").into_response(),
    };

    let path = std::path::Path::new(&bean_file);
    let parent = match path.parent() {
        Some(p) => p,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid BEANCOUNT_FILE path").into_response(),
    };

    let file_path = parent.join(filename);

    match tokio::fs::read_to_string(file_path).await {
        Ok(content) => (StatusCode::OK, Json(InfrastructureResponse { content })).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, format!("File not found: {}", e)).into_response(),
    }
}

async fn run_ledger(command: &str) -> String {
    // separate command into individual arguments
    let iter = command.split_whitespace();

    let mut ledger = Command::new("ledger");
    ledger.args(iter);

    // let output = ledger.status().await.expect("process failed to execute");
    let output = ledger.output().await.expect("failed to execute process");
    //let output = ledger.spawn().expect("ls command failed to start");

    // assert!(output.status.success());
    let result: String;

    if !output.status.success() {
        result = String::from_utf8_lossy(&output.stderr).to_string();
        // println!("not success: {}", result);
    } else {
       result = String::from_utf8_lossy(&output.stdout).to_string();
   }

    return result;
}

// #[instrument]
async fn shutdown() {
    let msg = "Shutting down on client request...";
    tracing::warn!(msg);
    //panic!("{}", msg);
    // println!("{}", msg);
    std::process::exit(0);
}
