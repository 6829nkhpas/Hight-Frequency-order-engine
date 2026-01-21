//! API module - HTTP and WebSocket endpoints.

pub mod orders;
pub mod websocket;

pub use orders::{get_order_book, health_check, submit_order};
pub use websocket::ws_handler;
