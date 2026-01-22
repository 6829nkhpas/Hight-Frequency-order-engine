//! Market data broadcasting module.

use crate::engine::EngineEvent;
use serde::Serialize;

/// Market data snapshot for broadcasting
#[derive(Debug, Clone, Serialize)]
pub struct MarketSnapshot {
    pub symbol: String,
    pub best_bid: Option<String>,
    pub best_ask: Option<String>,
    pub spread: Option<String>,
    pub last_trade_price: Option<String>,
    pub last_trade_quantity: Option<String>,
    pub timestamp: i64,
}

/// Convert engine event to market snapshot format
pub fn engine_event_to_snapshot(event: &EngineEvent, symbol: &str) -> Option<MarketSnapshot> {
    match event {
        EngineEvent::OrderBookUpdate(snapshot) => {
            let spread = match (snapshot.best_bid, snapshot.best_ask) {
                (Some(bid), Some(ask)) => Some((ask - bid).to_string()),
                _ => None,
            };

            Some(MarketSnapshot {
                symbol: symbol.to_string(),
                best_bid: snapshot.best_bid.map(|p| p.to_string()),
                best_ask: snapshot.best_ask.map(|p| p.to_string()),
                spread,
                last_trade_price: None,
                last_trade_quantity: None,
                timestamp: chrono::Utc::now().timestamp_millis(),
            })
        }
        EngineEvent::Trade(trade) => Some(MarketSnapshot {
            symbol: symbol.to_string(),
            best_bid: None,
            best_ask: None,
            spread: None,
            last_trade_price: Some(trade.price.to_string()),
            last_trade_quantity: Some(trade.quantity.to_string()),
            timestamp: trade.timestamp.timestamp_millis(),
        }),
    }
}
