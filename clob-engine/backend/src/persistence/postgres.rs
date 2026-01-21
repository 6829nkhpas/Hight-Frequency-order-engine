//! PostgreSQL persistence for trade journaling.

use crate::engine::{EngineEvent, EngineHandle, Trade};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use tokio::sync::mpsc;

/// Trade journaler that persists trades to PostgreSQL asynchronously
pub struct TradeJournaler {
    pool: PgPool,
    buffer: Vec<Trade>,
    buffer_size: usize,
    flush_interval: Duration,
}

impl TradeJournaler {
    /// Create a new trade journaler
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self {
            pool,
            buffer: Vec::with_capacity(100),
            buffer_size: 100,
            flush_interval: Duration::from_millis(100),
        })
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS trades (
                id UUID PRIMARY KEY,
                taker_order_id UUID NOT NULL,
                maker_order_id UUID NOT NULL,
                price DECIMAL NOT NULL,
                quantity DECIMAL NOT NULL,
                taker_side VARCHAR(4) NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS orders (
                id UUID PRIMARY KEY,
                side VARCHAR(4) NOT NULL,
                price DECIMAL NOT NULL,
                quantity DECIMAL NOT NULL,
                filled_quantity DECIMAL NOT NULL DEFAULT 0,
                status VARCHAR(20) NOT NULL DEFAULT 'open',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for common queries
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_trades_timestamp ON trades(timestamp DESC)",
        )
        .execute(&self.pool)
        .await?;

        tracing::info!("Database migrations completed");
        Ok(())
    }

    /// Start the journaler background task
    pub async fn run(mut self, handle: EngineHandle) {
        let mut events = handle.subscribe();
        let mut flush_interval = tokio::time::interval(self.flush_interval);

        tracing::info!("Trade journaler started");

        loop {
            tokio::select! {
                // Receive trade events
                result = events.recv() => {
                    match result {
                        Ok(EngineEvent::Trade(trade)) => {
                            self.buffer.push(trade);
                            if self.buffer.len() >= self.buffer_size {
                                self.flush().await;
                            }
                        }
                        Ok(_) => {} // Ignore non-trade events
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                            tracing::info!("Engine channel closed, flushing and exiting");
                            self.flush().await;
                            break;
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                            tracing::warn!("Journaler lagged behind by {} messages", n);
                        }
                    }
                }
                // Periodic flush
                _ = flush_interval.tick() => {
                    if !self.buffer.is_empty() {
                        self.flush().await;
                    }
                }
            }
        }
    }

    /// Flush buffered trades to the database
    async fn flush(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let trades: Vec<Trade> = self.buffer.drain(..).collect();
        let count = trades.len();

        // Batch insert trades
        for trade in trades {
            if let Err(e) = self.insert_trade(&trade).await {
                tracing::error!("Failed to persist trade {}: {}", trade.id, e);
                // In production, you'd want retry logic or a dead letter queue
            }
        }

        tracing::debug!("Flushed {} trades to database", count);
    }

    /// Insert a single trade
    async fn insert_trade(&self, trade: &Trade) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO trades (id, taker_order_id, maker_order_id, price, quantity, taker_side, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(trade.id)
        .bind(trade.taker_order_id)
        .bind(trade.maker_order_id)
        .bind(trade.price)
        .bind(trade.quantity)
        .bind(trade.taker_side.to_string())
        .bind(trade.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get recent trades from the database
    #[allow(dead_code)]
    pub async fn get_recent_trades(&self, limit: i64) -> Result<Vec<TradeRecord>, sqlx::Error> {
        let trades = sqlx::query_as::<_, TradeRecord>(
            r#"
            SELECT id, taker_order_id, maker_order_id, price, quantity, taker_side, timestamp
            FROM trades
            ORDER BY timestamp DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(trades)
    }
}

/// Trade record from the database
#[derive(Debug, sqlx::FromRow)]
pub struct TradeRecord {
    pub id: uuid::Uuid,
    pub taker_order_id: uuid::Uuid,
    pub maker_order_id: uuid::Uuid,
    pub price: rust_decimal::Decimal,
    pub quantity: rust_decimal::Decimal,
    pub taker_side: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Start a mock journaler that just logs trades (for testing without DB)
pub fn start_mock_journaler(handle: EngineHandle) -> mpsc::Sender<()> {
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
    let mut events = handle.subscribe();

    tokio::spawn(async move {
        tracing::info!("Mock trade journaler started (no database)");

        loop {
            tokio::select! {
                result = events.recv() => {
                    match result {
                        Ok(EngineEvent::Trade(trade)) => {
                            tracing::info!(
                                trade_id = %trade.id,
                                price = %trade.price,
                                quantity = %trade.quantity,
                                side = %trade.taker_side,
                                "Trade executed (mock journaler)"
                            );
                        }
                        Ok(_) => {}
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                        Err(_) => continue,
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("Mock journaler shutting down");
                    break;
                }
            }
        }
    });

    shutdown_tx
}
