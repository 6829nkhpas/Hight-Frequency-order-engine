//! WebSocket handler for real-time market data streaming.

use crate::engine::{EngineEvent, EngineHandle};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use std::sync::Arc;

/// WebSocket message sent to clients
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// A trade was executed
    Trade {
        price: String,
        quantity: String,
        side: String,
        timestamp: i64,
    },
    /// Order book update
    OrderBook {
        best_bid: Option<String>,
        best_ask: Option<String>,
        bids: Vec<[String; 2]>,
        asks: Vec<[String; 2]>,
    },
    /// Connection established
    Connected { message: String },
}

/// Handler for WebSocket upgrade requests
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(handle): State<Arc<EngineHandle>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, handle))
}

/// Handle an individual WebSocket connection
async fn handle_socket(socket: WebSocket, handle: Arc<EngineHandle>) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to engine events
    let mut events = handle.subscribe();

    // Send connected message
    let connected = WsMessage::Connected {
        message: "Connected to CLOB market data feed".to_string(),
    };
    if let Ok(json) = serde_json::to_string(&connected) {
        let _ = sender.send(Message::Text(json.into())).await;
    }

    // Spawn task to forward engine events to WebSocket
    let send_task = tokio::spawn(async move {
        loop {
            match events.recv().await {
                Ok(event) => {
                    let ws_msg = match event {
                        EngineEvent::Trade(trade) => WsMessage::Trade {
                            price: trade.price.to_string(),
                            quantity: trade.quantity.to_string(),
                            side: trade.taker_side.to_string(),
                            timestamp: trade.timestamp.timestamp_millis(),
                        },
                        EngineEvent::OrderBookUpdate(snapshot) => WsMessage::OrderBook {
                            best_bid: snapshot.best_bid.map(|p| p.to_string()),
                            best_ask: snapshot.best_ask.map(|p| p.to_string()),
                            bids: snapshot.bid_depth
                                .into_iter()
                                .map(|(p, q)| [p.to_string(), q.to_string()])
                                .collect(),
                            asks: snapshot.ask_depth
                                .into_iter()
                                .map(|(p, q)| [p.to_string(), q.to_string()])
                                .collect(),
                        },
                    };

                    if let Ok(json) = serde_json::to_string(&ws_msg) {
                        if sender.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            }
        }
    });

    // Handle incoming messages (pings, close, etc.)
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Close(_)) => break,
            Ok(Message::Ping(data)) => {
                // Pong is handled automatically by axum
                tracing::trace!("Received ping: {:?}", data);
            }
            Err(e) => {
                tracing::warn!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Abort the send task when client disconnects
    send_task.abort();
    tracing::debug!("WebSocket connection closed");
}
