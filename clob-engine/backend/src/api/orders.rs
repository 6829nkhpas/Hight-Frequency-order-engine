//! REST API for order submission.

use crate::engine::{EngineHandle, OrderRequest, Side};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Request body for submitting an order
#[derive(Debug, Deserialize)]
pub struct SubmitOrderRequest {
    /// "buy" or "sell"
    pub side: String,
    /// Limit price
    pub price: Decimal,
    /// Order quantity
    pub quantity: Decimal,
}

/// Response for a successful order submission
#[derive(Debug, Serialize)]
pub struct SubmitOrderResponse {
    pub success: bool,
    pub message: String,
    pub order_id: Option<Uuid>,
}

/// Submit a new order to the matching engine
pub async fn submit_order(
    State(handle): State<Arc<EngineHandle>>,
    Json(req): Json<SubmitOrderRequest>,
) -> impl IntoResponse {
    // Parse side
    let side = match req.side.to_lowercase().as_str() {
        "buy" => Side::Buy,
        "sell" => Side::Sell,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SubmitOrderResponse {
                    success: false,
                    message: "Invalid side. Must be 'buy' or 'sell'".to_string(),
                    order_id: None,
                }),
            );
        }
    };

    // Validate price and quantity
    if req.price <= Decimal::ZERO {
        return (
            StatusCode::BAD_REQUEST,
            Json(SubmitOrderResponse {
                success: false,
                message: "Price must be positive".to_string(),
                order_id: None,
            }),
        );
    }

    if req.quantity <= Decimal::ZERO {
        return (
            StatusCode::BAD_REQUEST,
            Json(SubmitOrderResponse {
                success: false,
                message: "Quantity must be positive".to_string(),
                order_id: None,
            }),
        );
    }

    let order_id = Uuid::new_v4();

    // Create order request
    let order_request = OrderRequest {
        side,
        price: req.price,
        quantity: req.quantity,
    };

    // Submit to engine
    match handle.submit_order(order_request).await {
        Ok(_) => (
            StatusCode::ACCEPTED,
            Json(SubmitOrderResponse {
                success: true,
                message: "Order submitted successfully".to_string(),
                order_id: Some(order_id),
            }),
        ),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(SubmitOrderResponse {
                success: false,
                message: "Engine unavailable".to_string(),
                order_id: None,
            }),
        ),
    }
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "clob-engine"
    }))
}

/// Get current order book state
pub async fn get_order_book(
    State(handle): State<Arc<EngineHandle>>,
) -> impl IntoResponse {
    let snapshot = handle.current_state.read().await;
    
    Json(serde_json::json!({
        "best_bid": snapshot.best_bid.map(|p| p.to_string()),
        "best_ask": snapshot.best_ask.map(|p| p.to_string()),
        "bids": snapshot.bid_depth.iter().map(|(p, q)| [p.to_string(), q.to_string()]).collect::<Vec<_>>(),
        "asks": snapshot.ask_depth.iter().map(|(p, q)| [p.to_string(), q.to_string()]).collect::<Vec<_>>(),
    }))
}
