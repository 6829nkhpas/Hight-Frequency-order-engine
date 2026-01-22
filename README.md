# High-Frequency Order Matching Engine

> **Production-grade CLOB (Central Limit Order Book) matching engine built in Rust**  
> Demonstrating sub-10μs latency order execution for high-frequency trading systems

[📖 Full Documentation](./clob-engine/README.md)

---

## Quick Overview

This repository contains a complete **trading engine** showcasing:

- ✅ **Ultra-low latency** order matching (<10μs average)
- ✅ **Rust backend** with zero-cost abstractions and memory safety
- ✅ **React/TypeScript frontend** with real-time market data
- ✅ **WebSocket streaming** for live updates
- ✅ **REST API** for order submission and queries
- ✅ **Performance benchmarking** suite included

---

## Project Structure

```
.
├── clob-engine/
│   ├── backend/          # Rust matching engine + API
│   ├── frontend/         # React/TypeScript UI
│   ├── README.md         # Detailed documentation
│   └── LICENSE           # MIT License
└── README.md             # This file
```

---

## Quick Start

### Backend (Rust)
```bash
cd clob-engine/backend
cargo run
# Starts on http://localhost:3000
```

### Frontend (React)
```bash
cd clob-engine/frontend
npm install
npm run dev
# Starts on http://localhost:5173
```

Visit **http://localhost:5173** to see the live trading interface!

---

## Key Features

| Feature | Description |
|---------|-------------|
| **Order Matching** | Price-time priority FIFO matching |
| **Real-Time Updates** | WebSocket streaming of trades & order book |
| **Performance Metrics** | Throughput and latency monitoring |
| **Modern UI** | Beautiful glassmorphism design with Tailwind CSS |
| **Type Safety** | Rust backend + TypeScript frontend |

---

## Performance at a Glance

- **Latency**: 3-10 μs average
- **Throughput**: 500-1,000 orders/sec
- **Memory**: ~15 MB for 10,000 active orders
- **P99 Latency**: <50 μs

---

## Tech Stack

**Backend:** Rust, Axum, Tokio, PostgreSQL  
**Frontend:** React 19, TypeScript, Vite, Tailwind CSS v4

---

## For HFT Companies

This project demonstrates production-ready skills in:
- Systems programming & performance engineering
- Lock-free concurrent architectures
- Financial systems (order matching, market microstructure)
- Full-stack development (Rust + React)

**[Read the full README](./clob-engine/README.md)** for technical deep-dive, architecture diagrams, API documentation, and more.

---

## License

MIT - See [LICENSE](./clob-engine/LICENSE)

---

## Contact

Looking for Rust backend developers for high-frequency trading systems?  
**Let's connect!**

- GitHub: [@6829nkhpas](https://github.com/6829nkhpas)
- Project: [High-Frequency Order Matching Engine](https://github.com/6829nkhpas/Hight-Frequency-order-engine)
