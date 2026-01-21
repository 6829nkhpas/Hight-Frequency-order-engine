//! Matching engine - single-threaded event loop for order processing.

use crate::engine::order::{Order, OrderRequest, Trade};
use crate::engine::order_book::OrderBook;
use tokio::sync::{broadcast, mpsc};

/// Events emitted by the matching engine
#[derive(Debug, Clone)]
pub enum EngineEvent {
    /// A trade was executed
    Trade(Trade),
    /// Order book state changed
    OrderBookUpdate {
        best_bid: Option<rust_decimal::Decimal>,
        best_ask: Option<rust_decimal::Decimal>,
        bid_depth: Vec<(rust_decimal::Decimal, rust_decimal::Decimal)>,
        ask_depth: Vec<(rust_decimal::Decimal, rust_decimal::Decimal)>,
    },
}

/// The matching engine processes orders and generates trades
pub struct MatchingEngine {
    /// The order book
    order_book: OrderBook,
    /// Channel to receive incoming orders
    order_rx: mpsc::Receiver<OrderRequest>,
    /// Channel to broadcast engine events (trades, updates)
    event_tx: broadcast::Sender<EngineEvent>,
    /// Number of depth levels to include in updates
    depth_levels: usize,
}

impl MatchingEngine {
    /// Create a new matching engine
    pub fn new(
        symbol: impl Into<String>,
        order_rx: mpsc::Receiver<OrderRequest>,
        event_tx: broadcast::Sender<EngineEvent>,
    ) -> Self {
        Self {
            order_book: OrderBook::new(symbol),
            order_rx,
            event_tx,
            depth_levels: 10,
        }
    }

    /// Run the matching engine event loop
    /// This should be spawned as a dedicated task
    pub async fn run(mut self) {
        tracing::info!("Matching engine started for {}", self.order_book.symbol);

        while let Some(request) = self.order_rx.recv().await {
            self.process_order(request);
        }

        tracing::info!("Matching engine shutting down");
    }

    /// Process a single order request
    fn process_order(&mut self, request: OrderRequest) {
        let order = Order::new(request.side, request.price, request.quantity);
        let order_id = order.id;

        tracing::debug!(
            order_id = %order_id,
            side = %order.side,
            price = %order.price,
            quantity = %order.quantity,
            "Processing order"
        );

        // Match the order against the book
        let trades = self.order_book.match_order(order);

        // Broadcast trades
        for trade in &trades {
            tracing::debug!(
                trade_id = %trade.id,
                price = %trade.price,
                quantity = %trade.quantity,
                "Trade executed"
            );

            // Ignore send errors (no subscribers)
            let _ = self.event_tx.send(EngineEvent::Trade(trade.clone()));
        }

        // Broadcast order book update
        self.broadcast_book_update();
    }

    /// Broadcast current order book state
    fn broadcast_book_update(&self) {
        let update = EngineEvent::OrderBookUpdate {
            best_bid: self.order_book.best_bid(),
            best_ask: self.order_book.best_ask(),
            bid_depth: self.order_book.bid_depth(self.depth_levels),
            ask_depth: self.order_book.ask_depth(self.depth_levels),
        };

        let _ = self.event_tx.send(update);
    }

    /// Get current order book statistics
    #[allow(dead_code)]
    pub fn stats(&self) -> EngineStats {
        EngineStats {
            symbol: self.order_book.symbol.clone(),
            best_bid: self.order_book.best_bid(),
            best_ask: self.order_book.best_ask(),
            spread: self.order_book.spread(),
            order_count: self.order_book.order_count(),
        }
    }
}

/// Engine statistics
#[derive(Debug, Clone)]
pub struct EngineStats {
    pub symbol: String,
    pub best_bid: Option<rust_decimal::Decimal>,
    pub best_ask: Option<rust_decimal::Decimal>,
    pub spread: Option<rust_decimal::Decimal>,
    pub order_count: usize,
}

/// Builder for creating the matching engine and its channels
pub struct EngineBuilder {
    symbol: String,
    order_buffer_size: usize,
    event_buffer_size: usize,
}

impl EngineBuilder {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_buffer_size: 10_000,
            event_buffer_size: 1_000,
        }
    }

    #[allow(dead_code)]
    pub fn order_buffer_size(mut self, size: usize) -> Self {
        self.order_buffer_size = size;
        self
    }

    #[allow(dead_code)]
    pub fn event_buffer_size(mut self, size: usize) -> Self {
        self.event_buffer_size = size;
        self
    }

    /// Build the engine and return handles for interaction
    pub fn build(self) -> (MatchingEngine, EngineHandle) {
        let (order_tx, order_rx) = mpsc::channel(self.order_buffer_size);
        let (event_tx, _) = broadcast::channel(self.event_buffer_size);

        let engine = MatchingEngine::new(self.symbol, order_rx, event_tx.clone());

        let handle = EngineHandle {
            order_tx,
            event_tx,
        };

        (engine, handle)
    }
}

/// Handle for interacting with the matching engine
#[derive(Clone)]
pub struct EngineHandle {
    /// Send orders to the engine
    pub order_tx: mpsc::Sender<OrderRequest>,
    /// Subscribe to engine events
    pub event_tx: broadcast::Sender<EngineEvent>,
}

impl EngineHandle {
    /// Submit an order to the engine
    pub async fn submit_order(&self, request: OrderRequest) -> Result<(), mpsc::error::SendError<OrderRequest>> {
        self.order_tx.send(request).await
    }

    /// Subscribe to engine events
    pub fn subscribe(&self) -> broadcast::Receiver<EngineEvent> {
        self.event_tx.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::order::Side;
    use rust_decimal_macros::dec;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_engine_processes_orders() {
        let (engine, handle) = EngineBuilder::new("BTC/USD").build();
        let mut events = handle.subscribe();

        // Spawn the engine
        tokio::spawn(engine.run());

        // Submit a sell order
        handle
            .submit_order(OrderRequest {
                side: Side::Sell,
                price: dec!(100),
                quantity: dec!(10),
            })
            .await
            .unwrap();

        // Wait for update
        let event = timeout(Duration::from_millis(100), events.recv())
            .await
            .unwrap()
            .unwrap();

        match event {
            EngineEvent::OrderBookUpdate { best_ask, .. } => {
                assert_eq!(best_ask, Some(dec!(100)));
            }
            _ => panic!("Expected OrderBookUpdate"),
        }
    }

    #[tokio::test]
    async fn test_engine_generates_trades() {
        let (engine, handle) = EngineBuilder::new("BTC/USD").build();
        let mut events = handle.subscribe();

        tokio::spawn(engine.run());

        // Submit a sell order
        handle
            .submit_order(OrderRequest {
                side: Side::Sell,
                price: dec!(100),
                quantity: dec!(10),
            })
            .await
            .unwrap();

        // Drain the book update
        let _ = timeout(Duration::from_millis(100), events.recv()).await;

        // Submit a matching buy order
        handle
            .submit_order(OrderRequest {
                side: Side::Buy,
                price: dec!(100),
                quantity: dec!(10),
            })
            .await
            .unwrap();

        // Should receive a trade event
        let event = timeout(Duration::from_millis(100), events.recv())
            .await
            .unwrap()
            .unwrap();

        match event {
            EngineEvent::Trade(trade) => {
                assert_eq!(trade.price, dec!(100));
                assert_eq!(trade.quantity, dec!(10));
            }
            _ => panic!("Expected Trade event"),
        }
    }
}
