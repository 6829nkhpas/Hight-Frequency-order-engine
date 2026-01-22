# High-Frequency Order Matching Engine (CLOB)

A high-performance **Central Limit Order Book (CLOB)** implemented in **Rust**, designed for **low-latency trading systems**.  
This project simulates the core architecture used in **fintech exchanges and HFT platforms**, with real-time market data streaming to a **React.js frontend**.

---

## 🚀 Features

### Core Engine
- **Lock-Free Matching Logic**: Single-threaded event loop with zero locks in hot path
- **Price-Time Priority**: Standard exchange matching rules
- **Async Order Ingestion**: Non-blocking Tokio channels
- **Real-Time WebSocket**: Live order book and trade streaming
- **Async Persistence**: PostgreSQL journaling without blocking the engine

### 🎯 NEW: Performance Simulation System
- **One-Click Benchmark**: Automated stress testing with random order generation
- **Real-Time Metrics Dashboard**: Live throughput, latency, and system performance visualization
- **Technical Insights**: Displays advantages over traditional C++/Java systems:
  - Lock-free architecture (zero mutex contention)
  - Memory safety (Rust ownership prevents race conditions)
  - Predictable latency (no GC pauses)
  - Async I/O (non-blocking persistence)

---

## 🏃 Quick Start

### Prerequisites
- **Rust** 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **Node.js** 18+ (`nvm install 18`)
- **PostgreSQL** (optional - mock journaler included)

### Backend
```bash
cd backend
cargo run
# Server starts on http://localhost:3000
```

### Frontend
```bash
cd frontend
npm install
npm run dev
# UI available at http://localhost:5173
```

---

## 📊 Using the Simulation System

1. **Start the Backend**: Ensure `cargo run` is active in `/backend`
2. **Open the Frontend**: Navigate to `http://localhost:5173`
3. **Run Simulation**: 
   - At the top of the page, you'll see the **Performance Simulation** panel
   - Enter the number of orders (default: 1000, max: 10000)
   - Click **"RUN SIMULATION"**
4. **View Results**:
   - **Throughput**: Orders processed per second
   - **Avg Latency**: Average order processing time in microseconds
   - **Min/Max Latency**: Performance bounds
   - **Historical Chart**: Track throughput across multiple runs
   - **Technical Advantages**: See why Rust outperforms legacy systems

---

## 🧩 API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health` | GET | Health check |
| `/api/orders` | POST | Submit order |
| `/api/orderbook` | GET | Get current order book snapshot |
| `/api/simulation` | POST | Run performance simulation |
| `/ws/market` | WS | Real-time market data stream |

### Simulation Request Example
```bash
curl -X POST http://localhost:3000/api/simulation \
  -H "Content-Type: application/json" \
  -d '{"num_orders": 5000}'
```

Response:
```json
{
  "success": true,
  "message": "Simulation completed: 5000 orders processed",
  "metrics": {
    "orders_submitted": 5000,
    "trades_executed": 0,
    "avg_latency_us": 45.67,
    "min_latency_us": 12,
    "max_latency_us": 892,
    "throughput_per_sec": 95432.1,
    "simulation_duration_ms": 52,
    "current_spread": "1.5",
    "total_volume_traded": "0"
  }
}
```

---

## 📈 Performance Goals

| Metric | Target | Status |
|--------|--------|--------|
| Order latency | < 100µs | ✅ Achieved |
| Throughput | 50k+ orders/sec | ✅ Verified in simulation |
| GC pauses | None | ✅ (Rust has no GC) |
| Locks in hot path | Zero | ✅ Single-threaded matcher |

---

## 🧠 System Architecture

```
┌──────────┐ WebSocket ┌──────────────┐
│ React UI │ ◄────────────────► │ API Gateway │
└──────────┘                    │ (Axum)      │
                                └─────┬────────┘
                                      │ Tokio Channel
                                      ▼
                         ┌───────────────────┐
                         │ Matching Engine   │
                         │ (In-Memory CLOB)  │
                         └─────┬────────────┘
                               │ Async
                               ▼
                       ┌────────────────────┐
                       │ Trade Journaler    │
                       │ (Postgres)         │
                       └────────────────────┘
```

---

## 🎨 Tech Stack

### Backend
- Rust 2021 Edition
- Tokio (async runtime)
- Axum (web framework)
- PostgreSQL + SQLx
- WebSockets

### Frontend
- React.js + TypeScript
- TailwindCSS v4
- Recharts (performance visualization)
- WebSocket API

---

## 🧪 Testing

```bash
# Run unit tests
cd backend
cargo test

# Run in release mode for benchmarking
cargo run --release
```

---

## 🏁 What This Demonstrates

1. **Exchange-Grade Architecture**: Simplified but correct implementation of a real CLOB
2. **Async System Design**: Event-driven, non-blocking I/O
3.  **Rust Mastery**: Ownership, concurrency, zero-cost abstractions
4. **Full-Stack Thinking**: Backend + Frontend integration
5. **Performance Engineering**: Sub-100μs latency, 50k+ ops/sec

---

## 📝 License

MIT License - feel free to use for learning and portfolio projects.
