/*!
 * CLI for Cashier Server
 */

use axum::response::AppendHeaders;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
extern crate base64;
use dotenvy::dotenv;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    dotenv().ok();
    initialize_logging();

    // no need to allow credentials explicitly
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(route_rust_ledger))
        .route("/hello", get(hello_img))
        .route("/ping", get(|| async { "pong" }))
        .route("/infrastructure", get(get_infrastructure))
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

async fn route_rust_ledger(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    log::debug!("rust_ledger: {:?}", params);

    if !params.contains_key("query") {
        return (
            StatusCode::BAD_REQUEST,
            [(
                axum::http::header::CONTENT_TYPE,
                "application/json",
            )],
            serde_json::json!({"error": "No query provided"}).to_string(),
        )
            .into_response();
    }

    let query = params["query"].as_str();

    let ledger_output = run_rust_ledger(query).await;

    // Return the raw JSON output directly with proper content type
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "application/json",
        )],
        ledger_output,
    )
        .into_response()
}

#[derive(Serialize)]
struct InfrastructureResponse {
    content: String,
}

#[derive(Deserialize)]
struct InfrastructureParams {
    file_path: String,
}

/**
 * Provides a Beancount infrastructure file.
 * For use with RustLedger.
 */
async fn get_infrastructure(Query(params): Query<InfrastructureParams>) -> impl IntoResponse {
    let filename = params.file_path;
    
    // Read the location from LEDGER_FILE environment variable.
    let bean_file = match std::env::var("LEDGER_FILE") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "LEDGER_FILE environment variable not set",
            )
                .into_response()
        }
    };

    // Search for the files in the pre-set directory only.
    let path = std::path::Path::new(&bean_file);
    let parent = match path.parent() {
        Some(p) => p,
        None => {
            log::error!("Invalid LEDGER_FILE path: {}", bean_file);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid LEDGER_FILE path",
            )
                .into_response()
        }
    };

    let file_path = parent.join(&filename);

    match tokio::fs::read_to_string(file_path).await {
        Ok(content) => (StatusCode::OK, Json(InfrastructureResponse { content })).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, format!("File not found: {}", e)).into_response(),
    }
}

/**
 * Run a Rust Ledger query using the rledger command-line tool.
 */
async fn run_rust_ledger(query: &str) -> String {
    // Get the ledger file from environment variable
    let bean_file = match std::env::var("LEDGER_FILE") {
        Ok(v) => v,
        Err(_) => return String::from("LEDGER_FILE environment variable not set"),
    };

    // Prepare command arguments: rledger query <file> "<query>" -f json
    let mut rledger = Command::new("rledger");
    rledger.args(["query", "-f", "json", &bean_file, query]);

    // Execute the command
    let output = rledger.output().await.expect("failed to execute process");

    let result: String;
    if !output.status.success() {
        result = String::from_utf8_lossy(&output.stderr).to_string();
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
