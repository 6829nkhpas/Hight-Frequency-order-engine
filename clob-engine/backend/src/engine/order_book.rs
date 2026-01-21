//! Order book implementation using BTreeMap for price levels.

use crate::engine::order::{Order, Side, Trade};
use rust_decimal::Decimal;
use std::collections::{BTreeMap, VecDeque};

/// A price level in the order book containing orders at that price
#[derive(Debug, Default)]
pub struct PriceLevel {
    /// Orders at this price level, ordered by time (FIFO)
    pub orders: VecDeque<Order>,
    /// Total quantity at this price level
    pub total_quantity: Decimal,
}

impl PriceLevel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an order to this price level
    pub fn add_order(&mut self, order: Order) {
        self.total_quantity += order.remaining_quantity;
        self.orders.push_back(order);
    }

    /// Remove the front order (oldest by time priority)
    pub fn pop_front(&mut self) -> Option<Order> {
        if let Some(order) = self.orders.pop_front() {
            self.total_quantity -= order.remaining_quantity;
            Some(order)
        } else {
            None
        }
    }

    /// Get mutable reference to front order
    pub fn front_mut(&mut self) -> Option<&mut Order> {
        self.orders.front_mut()
    }

    /// Check if this price level is empty
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    /// Get the number of orders at this level
    pub fn len(&self) -> usize {
        self.orders.len()
    }
}

/// The central limit order book
#[derive(Debug)]
pub struct OrderBook {
    /// Buy orders: highest price first (descending)
    /// Key is negated price for reverse ordering
    bids: BTreeMap<Decimal, PriceLevel>,
    
    /// Sell orders: lowest price first (ascending)
    asks: BTreeMap<Decimal, PriceLevel>,
    
    /// Symbol for this order book
    pub symbol: String,
}

impl OrderBook {
    /// Create a new order book for the given symbol
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            symbol: symbol.into(),
        }
    }

    /// Get the best bid price (highest buy price)
    pub fn best_bid(&self) -> Option<Decimal> {
        self.bids.keys().next_back().copied()
    }

    /// Get the best ask price (lowest sell price)
    pub fn best_ask(&self) -> Option<Decimal> {
        self.asks.keys().next().copied()
    }

    /// Get the spread between best bid and ask
    pub fn spread(&self) -> Option<Decimal> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get the bid side depth (price levels and quantities)
    pub fn bid_depth(&self, levels: usize) -> Vec<(Decimal, Decimal)> {
        self.bids
            .iter()
            .rev()
            .take(levels)
            .map(|(price, level)| (*price, level.total_quantity))
            .collect()
    }

    /// Get the ask side depth (price levels and quantities)
    pub fn ask_depth(&self, levels: usize) -> Vec<(Decimal, Decimal)> {
        self.asks
            .iter()
            .take(levels)
            .map(|(price, level)| (*price, level.total_quantity))
            .collect()
    }

    /// Add an order to the book (no matching, just insertion)
    pub fn add_order(&mut self, order: Order) {
        let book = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book.entry(order.price)
            .or_insert_with(PriceLevel::new)
            .add_order(order);
    }

    /// Match an incoming order against the book
    /// Returns a vector of trades generated
    pub fn match_order(&mut self, mut incoming: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        // Get the opposing book
        let opposing_book = match incoming.side {
            Side::Buy => &mut self.asks,
            Side::Sell => &mut self.bids,
        };

        // Keep matching while possible
        loop {
            if incoming.is_filled() {
                break;
            }

            // Get the best opposing price
            let best_price = match incoming.side {
                Side::Buy => opposing_book.keys().next().copied(),
                Side::Sell => opposing_book.keys().next_back().copied(),
            };

            let best_price = match best_price {
                Some(p) => p,
                None => break, // No orders on opposing side
            };

            // Check if prices cross
            let prices_cross = match incoming.side {
                Side::Buy => incoming.price >= best_price,
                Side::Sell => incoming.price <= best_price,
            };

            if !prices_cross {
                break;
            }

            // Get the price level
            let level = opposing_book.get_mut(&best_price).unwrap();

            // Match against orders at this level
            while !incoming.is_filled() && !level.is_empty() {
                let maker = level.front_mut().unwrap();

                // Calculate fill quantity
                let fill_qty = incoming.remaining_quantity.min(maker.remaining_quantity);

                // Create trade (execute at maker's price)
                let trade = Trade::new(
                    incoming.id,
                    maker.id,
                    best_price, // Trade at the maker's price
                    fill_qty,
                    incoming.side,
                );

                // Update quantities
                incoming.fill(fill_qty);
                maker.fill(fill_qty);

                // Update level total
                level.total_quantity -= fill_qty;

                // Remove filled maker order
                if maker.is_filled() {
                    level.pop_front();
                }

                trades.push(trade);
            }

            // Remove empty price level
            if level.is_empty() {
                match incoming.side {
                    Side::Buy => opposing_book.remove(&best_price),
                    Side::Sell => opposing_book.remove(&best_price),
                };
            }
        }

        // If incoming order has remaining quantity, add to book
        if !incoming.is_filled() {
            self.add_order(incoming);
        }

        trades
    }

    /// Get total number of orders in the book
    pub fn order_count(&self) -> usize {
        let bid_count: usize = self.bids.values().map(|l| l.len()).sum();
        let ask_count: usize = self.asks.values().map(|l| l.len()).sum();
        bid_count + ask_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_empty_order_book() {
        let book = OrderBook::new("BTC/USD");
        assert!(book.best_bid().is_none());
        assert!(book.best_ask().is_none());
        assert!(book.spread().is_none());
    }

    #[test]
    fn test_add_orders() {
        let mut book = OrderBook::new("BTC/USD");
        
        book.add_order(Order::new(Side::Buy, dec!(100), dec!(10)));
        book.add_order(Order::new(Side::Buy, dec!(99), dec!(5)));
        book.add_order(Order::new(Side::Sell, dec!(101), dec!(8)));
        
        assert_eq!(book.best_bid(), Some(dec!(100)));
        assert_eq!(book.best_ask(), Some(dec!(101)));
        assert_eq!(book.spread(), Some(dec!(1)));
    }

    #[test]
    fn test_full_match() {
        let mut book = OrderBook::new("BTC/USD");
        
        // Add a sell order
        book.add_order(Order::new(Side::Sell, dec!(100), dec!(10)));
        
        // Submit a matching buy order
        let buy = Order::new(Side::Buy, dec!(100), dec!(10));
        let trades = book.match_order(buy);
        
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, dec!(10));
        assert_eq!(trades[0].price, dec!(100));
        assert!(book.best_ask().is_none()); // Sell order fully filled
    }

    #[test]
    fn test_partial_match() {
        let mut book = OrderBook::new("BTC/USD");
        
        // Add a sell order for 10 units
        book.add_order(Order::new(Side::Sell, dec!(100), dec!(10)));
        
        // Submit a buy order for 5 units
        let buy = Order::new(Side::Buy, dec!(100), dec!(5));
        let trades = book.match_order(buy);
        
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, dec!(5));
        
        // 5 units remaining on the sell side
        let depth = book.ask_depth(10);
        assert_eq!(depth.len(), 1);
        assert_eq!(depth[0], (dec!(100), dec!(5)));
    }

    #[test]
    fn test_price_time_priority() {
        let mut book = OrderBook::new("BTC/USD");
        
        // Add two sell orders at same price
        let sell1 = Order::new(Side::Sell, dec!(100), dec!(5));
        let sell1_id = sell1.id;
        book.add_order(sell1);
        
        let sell2 = Order::new(Side::Sell, dec!(100), dec!(5));
        book.add_order(sell2);
        
        // Buy should match with first (older) order
        let buy = Order::new(Side::Buy, dec!(100), dec!(5));
        let trades = book.match_order(buy);
        
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].maker_order_id, sell1_id); // First order matched
    }

    #[test]
    fn test_no_match_spread() {
        let mut book = OrderBook::new("BTC/USD");
        
        // Add sell at 101
        book.add_order(Order::new(Side::Sell, dec!(101), dec!(10)));
        
        // Buy at 100 should not match
        let buy = Order::new(Side::Buy, dec!(100), dec!(10));
        let trades = book.match_order(buy);
        
        assert!(trades.is_empty());
        assert_eq!(book.best_bid(), Some(dec!(100))); // Buy added to book
        assert_eq!(book.best_ask(), Some(dec!(101))); // Sell still there
    }

    #[test]
    fn test_aggressive_matching_multiple_levels() {
        let mut book = OrderBook::new("BTC/USD");
        
        // Add sells at multiple prices
        book.add_order(Order::new(Side::Sell, dec!(100), dec!(5)));
        book.add_order(Order::new(Side::Sell, dec!(101), dec!(5)));
        book.add_order(Order::new(Side::Sell, dec!(102), dec!(5)));
        
        // Buy order that sweeps through multiple levels
        let buy = Order::new(Side::Buy, dec!(102), dec!(12));
        let trades = book.match_order(buy);
        
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, dec!(100)); // Best price first
        assert_eq!(trades[1].price, dec!(101));
        assert_eq!(trades[2].price, dec!(102));
        assert_eq!(trades[2].quantity, dec!(2)); // Partial fill at last level
    }
}
