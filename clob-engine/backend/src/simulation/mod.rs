//! Performance simulation and metrics tracking.

use crate::engine::{EngineHandle, OrderRequest, Side};
use rand::Rng;
use rust_decimal::Decimal;
use serde::Serialize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Performance metrics tracked during simulation
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceMetrics {
    pub orders_submitted: u64,
    pub trades_executed: u64,
    pub avg_latency_us: f64,
    pub min_latency_us: u64,
    pub max_latency_us: u64,
    pub throughput_per_sec: f64,
    pub simulation_duration_ms: u64,
    pub current_spread: Option<String>,
    pub total_volume_traded: String,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            orders_submitted: 0,
            trades_executed: 0,
            avg_latency_us: 0.0,
            min_latency_us: u64::MAX,
            max_latency_us: 0,
            throughput_per_sec: 0.0,
            simulation_duration_ms: 0,
            current_spread: None,
            total_volume_traded: "0".to_string(),
        }
    }
}

/// Simulation configuration
pub struct SimulationConfig {
    pub num_orders: u64,
    pub base_price: Decimal,
    pub price_variance: Decimal,
    pub min_quantity: Decimal,
    pub max_quantity: Decimal,
    pub delay_between_orders_us: u64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            num_orders: 1000,
            base_price: Decimal::new(10000, 2), // 100.00
            price_variance: Decimal::new(500, 2), // 5.00
            min_quantity: Decimal::new(100, 4), // 0.0100
            max_quantity: Decimal::new(10000, 4), // 1.0000
            delay_between_orders_us: 100, // 100 microseconds between orders
        }
    }
}

/// Simulation runner
pub struct Simulator {
    handle: Arc<EngineHandle>,
    metrics: Arc<RwLock<PerformanceMetrics>>,
}

impl Simulator {
    pub fn new(handle: Arc<EngineHandle>) -> Self {
        Self {
            handle,
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
        }
    }

    /// Run a simulation with the given configuration
    pub async fn run_simulation(&self, config: SimulationConfig) -> PerformanceMetrics {
        let mut rng = rand::thread_rng();
        let start_time = Instant::now();
        let mut latencies = Vec::with_capacity(config.num_orders as usize);
        
        // Reset metrics
        {
            let mut metrics = self.metrics.write().await;
            *metrics = PerformanceMetrics::default();
        }

        tracing::info!(
            "Starting simulation: {} orders, base_price={}, variance={}",
            config.num_orders,
            config.base_price,
            config.price_variance
        );

        for i in 0..config.num_orders {
            // Generate random order
            let side = if rng.gen_bool(0.5) {
                Side::Buy
            } else {
                Side::Sell
            };

            // Random price around base price
            let price_offset = Decimal::new(
                rng.gen_range(-config.price_variance.mantissa()..=config.price_variance.mantissa()),
                config.price_variance.scale(),
            );
            let price = config.base_price + price_offset;

            // Random quantity
            let quantity = Decimal::new(
                rng.gen_range(config.min_quantity.mantissa()..=config.max_quantity.mantissa()),
                config.max_quantity.scale(),
            );

            let order = OrderRequest {
                side,
                price,
                quantity,
            };

            // Measure order submission latency
            let order_start = Instant::now();
            let _ = self.handle.submit_order(order).await;
            let order_latency = order_start.elapsed();

            latencies.push(order_latency.as_micros() as u64);

            // Small delay to simulate realistic order flow
            if config.delay_between_orders_us > 0 {
                tokio::time::sleep(Duration::from_micros(config.delay_between_orders_us)).await;
            }

            // Update progress every 100 orders
            if (i + 1) % 100 == 0 {
                tracing::debug!("Simulation progress: {}/{} orders", i + 1, config.num_orders);
            }
        }

        let total_duration = start_time.elapsed();
        
        // Calculate metrics
        let avg_latency_us = latencies.iter().sum::<u64>() as f64 / latencies.len() as f64;
        let min_latency_us = *latencies.iter().min().unwrap_or(&0);
        let max_latency_us = *latencies.iter().max().unwrap_or(&0);
        let throughput_per_sec = config.num_orders as f64 / total_duration.as_secs_f64();

        // Get current order book state
        let snapshot = self.handle.current_state.read().await;
        let current_spread = match (snapshot.best_bid, snapshot.best_ask) {
            (Some(bid), Some(ask)) => Some((ask - bid).to_string()),
            _ => None,
        };

        let final_metrics = PerformanceMetrics {
            orders_submitted: config.num_orders,
            trades_executed: 0, // Would need to track from events
            avg_latency_us,
            min_latency_us,
            max_latency_us,
            throughput_per_sec,
            simulation_duration_ms: total_duration.as_millis() as u64,
            current_spread,
            total_volume_traded: "0".to_string(), // Would need to track from events
        };

        // Update shared metrics
        {
            let mut metrics = self.metrics.write().await;
            *metrics = final_metrics.clone();
        }

        tracing::info!(
            "Simulation complete: {} orders in {}ms, throughput={:.2} orders/sec, avg_latency={:.2}μs",
            config.num_orders,
            total_duration.as_millis(),
            throughput_per_sec,
            avg_latency_us
        );

        final_metrics
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().await.clone()
    }
}
