# Akawo3

> Traditional Savings. Modern Trust.

Akawo3 is an open-source decentralized rotating savings platform inspired by the traditional Akawo, Esusu, and Ajo model. It brings community-driven savings into a transparent, auditable, and modern digital experience using Rust, Stellar Soroban, Axum, Dioxus, PostgreSQL, and Valkey.

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Built%20with-Rust-orange.svg" alt="Built with Rust" /></a>
  <a href="https://soroban.stellar.org/"><img src="https://img.shields.io/badge/Powered%20by-Soroban-blue.svg" alt="Powered by Soroban" /></a>
</p>

## Why Akawo3?

Akawo3 exists to make traditional savings circles more trustworthy, more accessible, and more resilient in the digital age. By combining blockchain technology with strong engineering practices, the project aims to demonstrate how modern fintech systems can support real-world community finance.

## Highlights

- Transparent and auditable savings flows
- Community-based rotating savings groups
- Wallet-based authentication and secure transactions
- Production-minded backend and frontend architecture
- Open-source development workflow with testing and security in mind

## Tech Stack

- Rust
- Axum
- Tokio
- SQLx and PostgreSQL
- Valkey
- Dioxus
- Stellar and Soroban
- Docker and Docker Compose
- GitHub Actions

## Project Structure

```text
akawo3/
├── apps/
│   ├── api/
│   ├── web/
│   └── worker/
├── contracts/
├── crates/
├── deployments/
├── docs/
└── scripts/
```

## Architecture Overview

The platform is designed as a layered system that combines a modern web frontend, secure backend services, blockchain contracts, and supporting infrastructure.

```text
                    Internet
                        │
                  Cloudflare
                        │
                 Caddy / Nginx
                        │
                Docker Compose
                        │
      ┌─────────────────┼─────────────────┐
      │                 │                 │
  Dioxus UI        Axum API         Worker Service
      │                 │                 │
      │          ┌──────┴──────┐          │
      │          │             │          │
      ▼          ▼             ▼          ▼
 Wallet Auth PostgreSQL     Valkey     Email Queue
      │
      ▼
 Stellar RPC
      │
      ▼
 Soroban Smart Contract
      │
      ▼
 Stellar Ledger
```

## Getting Started

### Prerequisites

- Rust toolchain
- Docker and Docker Compose
- PostgreSQL and Valkey, or container-based services
- Stellar CLI for smart contract work

### Quick Start

```bash
git clone https://github.com/your-org/akawo3.git
cd akawo3
docker compose up
```

## Development Roadmap

- Foundation and architecture
- Backend APIs and services
- Database design and migrations
- Blockchain and Soroban integration
- Frontend experience and wallet flows
- Security hardening, observability, and deployment

## Contributing

Contributions are welcome. If you would like to improve the platform, fix a bug, add a feature, or improve documentation, please open an issue or submit a pull request.

## License

Akawo3 is released under the MIT License. You are free to use, copy, modify, merge, publish, distribute, and sublicense this software, provided the original copyright notice and license text are included.

See [LICENSE](LICENSE) for details.


# Stretch Goals

- Mobile App
- Kubernetes
- Terraform
- GraphQL
- Event Sourcing
- CQRS
- Multi-tenancy
- Treasury
- DAO Governance
- AI Financial Assistant
- Offline-first support
- Cross-chain interoperability

---

# Success Criteria

By Day 80, Akawo3 should include:

- ✅ Production backend
- ✅ Production frontend
- ✅ Smart contracts
- ✅ Wallet authentication
- ✅ NFT subsystem
- ✅ Reputation system
- ✅ PostgreSQL
- ✅ Valkey
- ✅ Docker
- ✅ GitHub Actions
- ✅ DevSecOps
- ✅ CI/CD
- ✅ Monitoring
- ✅ Documentation
- ✅ Automated deployment
- ✅ Portfolio-quality architecture

---

# Long-Term Vision

Akawo3 is more than a portfolio project.

This is an opportunity to learn how production-grade fintech systems are designed, built, secured, tested, deployed, and maintained.

The ultimate goal is not merely to complete the application, but to become a **Rust Backend Engineer**, **Blockchain Engineer**, and **Software Engineer** capable of designing and delivering production-ready systems with confidence.