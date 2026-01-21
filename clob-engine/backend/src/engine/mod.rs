//! Engine module - Core matching engine and order book logic.

pub mod matcher;
pub mod order;
pub mod order_book;

pub use matcher::{EngineBuilder, EngineEvent, EngineHandle, MatchingEngine};
pub use order::{Order, OrderRequest, OrderStatus, Side, Trade};
pub use order_book::OrderBook;
