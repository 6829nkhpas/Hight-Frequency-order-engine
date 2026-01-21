//! Core order type definitions for the CLOB engine.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Order side - Buy or Sell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Buy => write!(f, "buy"),
            Side::Sell => write!(f, "sell"),
        }
    }
}

/// Order status in the book
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}

/// A limit order in the order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Unique order identifier
    pub id: Uuid,
    /// Buy or Sell
    pub side: Side,
    /// Limit price
    pub price: Decimal,
    /// Original quantity
    pub quantity: Decimal,
    /// Remaining unfilled quantity
    pub remaining_quantity: Decimal,
    /// Order creation timestamp (used for time priority)
    pub timestamp: DateTime<Utc>,
    /// Current order status
    pub status: OrderStatus,
}

impl Order {
    /// Create a new order with the given parameters
    pub fn new(side: Side, price: Decimal, quantity: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            side,
            price,
            quantity,
            remaining_quantity: quantity,
            timestamp: Utc::now(),
            status: OrderStatus::Open,
        }
    }

    /// Check if this order can match with another order
    pub fn can_match(&self, other: &Order) -> bool {
        match (self.side, other.side) {
            (Side::Buy, Side::Sell) => self.price >= other.price,
            (Side::Sell, Side::Buy) => self.price <= other.price,
            _ => false, // Same side orders can't match
        }
    }

    /// Fill this order by the given quantity
    pub fn fill(&mut self, qty: Decimal) {
        self.remaining_quantity -= qty;
        if self.remaining_quantity.is_zero() {
            self.status = OrderStatus::Filled;
        } else {
            self.status = OrderStatus::PartiallyFilled;
        }
    }

    /// Check if this order is fully filled
    pub fn is_filled(&self) -> bool {
        self.remaining_quantity.is_zero()
    }
}

/// A trade execution between two orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Unique trade identifier
    pub id: Uuid,
    /// The aggressive (taker) order ID
    pub taker_order_id: Uuid,
    /// The passive (maker) order ID
    pub maker_order_id: Uuid,
    /// Execution price (maker's price)
    pub price: Decimal,
    /// Executed quantity
    pub quantity: Decimal,
    /// Taker side (the side that initiated the trade)
    pub taker_side: Side,
    /// Trade execution timestamp
    pub timestamp: DateTime<Utc>,
}

impl Trade {
    /// Create a new trade
    pub fn new(
        taker_order_id: Uuid,
        maker_order_id: Uuid,
        price: Decimal,
        quantity: Decimal,
        taker_side: Side,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            taker_order_id,
            maker_order_id,
            price,
            quantity,
            taker_side,
            timestamp: Utc::now(),
        }
    }
}

/// Request to submit a new order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub side: Side,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_order_creation() {
        let order = Order::new(Side::Buy, dec!(100.50), dec!(10));
        assert_eq!(order.side, Side::Buy);
        assert_eq!(order.price, dec!(100.50));
        assert_eq!(order.quantity, dec!(10));
        assert_eq!(order.remaining_quantity, dec!(10));
        assert_eq!(order.status, OrderStatus::Open);
    }

    #[test]
    fn test_order_can_match() {
        let buy = Order::new(Side::Buy, dec!(100), dec!(10));
        let sell = Order::new(Side::Sell, dec!(99), dec!(5));
        
        assert!(buy.can_match(&sell)); // Buy at 100 can match sell at 99
        assert!(sell.can_match(&buy)); // Sell at 99 can match buy at 100
    }

    #[test]
    fn test_order_cannot_match_same_side() {
        let buy1 = Order::new(Side::Buy, dec!(100), dec!(10));
        let buy2 = Order::new(Side::Buy, dec!(99), dec!(5));
        
        assert!(!buy1.can_match(&buy2));
    }

    #[test]
    fn test_order_fill() {
        let mut order = Order::new(Side::Buy, dec!(100), dec!(10));
        
        order.fill(dec!(5));
        assert_eq!(order.remaining_quantity, dec!(5));
        assert_eq!(order.status, OrderStatus::PartiallyFilled);
        
        order.fill(dec!(5));
        assert_eq!(order.remaining_quantity, dec!(0));
        assert_eq!(order.status, OrderStatus::Filled);
        assert!(order.is_filled());
    }
}
