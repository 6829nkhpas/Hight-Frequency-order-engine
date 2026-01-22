//! Simulation API endpoints.

use crate::engine::EngineHandle;
use crate::simulation::Simulator;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Request to start a simulation
#[derive(Debug, Deserialize)]
pub struct SimulationRequest {
    #[serde(default = "default_num_orders")]
    pub num_orders: u64,
}

fn default_num_orders() -> u64 {
    1000
}

/// Response with simulation results
#[derive(Debug, Serialize)]
pub struct SimulationResponse {
    pub success: bool,
    pub message: String,
    pub metrics: Option<crate::simulation::PerformanceMetrics>,
}

/// Start a performance simulation
#[axum::debug_handler]
pub async fn run_simulation(
    State(handle): State<Arc<EngineHandle>>,
    Json(req): Json<SimulationRequest>,
) -> Json<SimulationResponse> {
    // Create simulator
    let simulator = Simulator::new(handle);
    
    // Configure simulation
    let mut config = crate::simulation::SimulationConfig::default();
    config.num_orders = req.num_orders.min(10000); // Cap at 10k orders for safety

    tracing::info!("Starting simulation with {} orders", config.num_orders);

    // Run simulation
    let metrics = simulator.run_simulation(config).await;

    Json(SimulationResponse {
        success: true,
        message: format!("Simulation completed: {} orders processed", req.num_orders),
        metrics: Some(metrics),
    })
}
