use axum::{Router, routing::get};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initializing the logging system
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, Dev!" }))
        .route("/name", get(|| async { "Hello, Vishnu!" }))
        .layer(TraceLayer::new_for_http()) // log every request
        .layer(CompressionLayer::new()); // automatically zips the response

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("🚀 Server with Memory running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
