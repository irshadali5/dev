## Overview

This project was built to test the limits of **Rust** in a high-throughput e-commerce scenario. Using `axum` and `sqlx`, the API handles thousands of concurrent requests with minimal resource usage.

### Key Features
- JWT Authentication
- Stripe Integration
- Real-time inventory tracking

### Architecture
The backend is split into multiple crates mimicking a Clean Architecture pattern, allowing for easy testing and domain isolation.
