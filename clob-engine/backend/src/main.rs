//! CLOB Engine - High-Frequency Order Matching Engine
//!
//! A Central Limit Order Book implementation in Rust with:
//! - Single-threaded matching engine (no locks in hot path)
//! - Async order ingestion via Tokio channels
//! - Real-time WebSocket market data streaming
//! - Async trade persistence to PostgreSQL

mod api;
mod broadcast;
mod engine;
mod persistence;

use api::{get_order_book, health_check, submit_order, ws_handler};
use axum::{
    routing::{get, post},
    Router,
};
use engine::EngineBuilder;
use persistence::start_mock_journaler;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "clob_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting CLOB Engine...");

    // Build the matching engine
    let (engine, handle) = EngineBuilder::new("BTC/USD").build();
    let handle = Arc::new(handle);

    // Spawn the matching engine task
    tokio::spawn(engine.run());

    // Start mock journaler (use TradeJournaler for real DB)
    // To use real PostgreSQL:
    // let journaler = TradeJournaler::new(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    // journaler.run_migrations().await.unwrap();
    // tokio::spawn(journaler.run((*handle).clone()));
    let _journaler_shutdown = start_mock_journaler((*handle).clone());

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        // REST API
        .route("/api/health", get(health_check))
        .route("/api/orders", post(submit_order))
        .route("/api/orderbook", get(get_order_book))
        // WebSocket
        .route("/ws/market", get(ws_handler))
        .layer(cors)
        .with_state(handle);

    // Start the server
    let addr = "0.0.0.0:3000";
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
