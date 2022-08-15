use axum::{
    routing::{get,post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    //println!("Hello, world!");

    // tracing init
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "cashier-server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new().allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors);

    let address = std::net::SocketAddr::from(([0,0,0,0], 3000));
    tracing::debug!("listening on {}", address);

    // run it with hyper on localhost:3000
    // &"0.0.0.0:3000".parse().unwrap()
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
