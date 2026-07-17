# Akawo3

> **Traditional Savings. Modern Trust.**
>
> A production-grade decentralized rotating savings platform built with **Rust**, **Soroban (Stellar)**, **Axum**, **Dioxus**, **PostgreSQL**, **Valkey**, and modern **DevSecOps** practices.

---

# Vision

Akawo3 is a decentralized implementation of the traditional **Akawo/Esusu/Ajo** rotating savings model.

Rather than simply creating another blockchain application, Akawo3 aims to demonstrate how modern software engineering, fintech architecture, and blockchain technology can solve real-world community finance problems.

This repository doubles as:

- a production-ready portfolio project
- an engineering journal
- a learning roadmap
- a showcase of backend, blockchain, frontend and DevSecOps skills

---

# Mission

Build a production-grade fintech platform from scratch in **80 days** while mastering:

- Rust
- Backend Engineering
- Blockchain Engineering
- Soroban Smart Contracts
- System Design
- DevSecOps
- CI/CD
- Docker
- Production Deployment

---

# Project Goals

By the end of this project, Akawo3 should demonstrate experience with:

- Production Rust
- Clean Architecture
- Domain Driven Design
- Event Driven Architecture
- REST APIs
- Wallet Authentication
- Stellar Soroban
- PostgreSQL
- Valkey
- Dioxus
- Docker
- GitHub Actions
- DevSecOps
- Observability
- Infrastructure
- Testing
- Documentation

---

# Final Product

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

---

# Technology Stack

## Language

- Rust

## Backend

- Axum
- Tokio
- SQLx
- PostgreSQL
- Valkey
- Tower
- Serde
- utoipa
- Tracing
- JWT
- Argon2

## Blockchain

- Stellar
- Soroban SDK
- Stellar CLI
- Soroban CLI
- RPC

## Frontend

- Dioxus
- Dioxus Router
- Signals

## Infrastructure

- Docker
- Docker Compose
- GitHub Actions
- Caddy/Nginx

## DevSecOps

- cargo audit
- cargo deny
- Trivy
- Dependabot
- Secret Scanning
- SBOM

---

# Repository Structure

```

akawo3/

apps/
api/
web/
worker/
admin/

contracts/
akawo3-contract/

crates/
domain/
application/
infrastructure/
shared/
config/
sdk/

deployments/
docker/
compose/
github/

docs/
architecture/
adr/
api/
diagrams/

scripts/

```

---

# Learning Philosophy

This project is built around one principle:

> Learn by building production software.

Every module follows this process.

1. Theory
2. Architecture
3. Implementation
4. Testing
5. Security
6. Documentation
7. Deployment

---

# 80-Day Curriculum

---

# Phase 0 — Foundation

**Duration**

Days 1–3

## Objective

Set up the project like a professional engineering team.

## Topics

- Git
- GitHub Flow
- Conventional Commits
- Rust Workspace
- ADRs
- Project Planning
- README
- Issue Templates
- Pull Requests

## Deliverables

- Repository created
- Rust workspace initialized
- Folder structure completed
- GitHub Project Board
- Branch protection rules
- Initial ADR

---

# Phase 1 — Rust Engineering

**Duration**

Days 4–10

## Objective

Become comfortable writing production Rust.

## Topics

- Ownership
- Borrowing
- Lifetimes
- Traits
- Generics
- Enums
- Pattern Matching
- Error Handling
- Async
- Tokio
- Modules
- Macros
- Testing

## Mini Projects

- CLI
- Config Loader
- File Parser
- Async Downloader

## Deliverables

- Rust fundamentals mastered
- Utility crates created
- Test suite

---

# Phase 2 — Backend Engineering

**Duration**

Days 11–20

## Objective

Build a production REST API.

## Topics

- Axum
- Tower
- Middleware
- Dependency Injection
- DTOs
- Validation
- Repository Pattern
- Service Layer
- Error Handling
- OpenAPI
- Logging
- Configuration

## Deliverables

- REST API
- Swagger Documentation
- Structured Logging
- Clean Architecture

---

# Phase 3 — PostgreSQL

**Duration**

Days 21–24

## Topics

- Database Design
- Normalization
- Constraints
- Indexes
- SQLx
- Transactions
- Connection Pooling
- Migrations

## Deliverables

- Production database schema
- Migration scripts
- Seed data

---

# Phase 4 — Valkey

**Duration**

Days 25–27

## Topics

- Caching
- Sessions
- Rate Limiting
- Pub/Sub
- Locks
- Queues

## Deliverables

- Cache layer
- Rate limiter
- Session store

---

# Phase 5 — Docker

**Duration**

Days 28–31

## Topics

- Images
- Containers
- Networks
- Volumes
- Multi-stage Builds
- Health Checks
- Docker Compose

## Deliverables

Everything runs with

```

docker compose up

```

---

# Phase 6 — CI/CD

**Duration**

Days 32–35

## Topics

- GitHub Actions
- Testing
- Formatting
- Linting
- Build Pipeline
- Docker Build
- Deployment

## Deliverables

CI Pipeline

```

Push

↓

cargo fmt

↓

cargo clippy

↓

cargo test

↓

docker build

↓

deploy

```

---

# Phase 7 — DevSecOps

**Duration**

Days 36–40

## Topics

- cargo audit
- cargo deny
- Secret Management
- Supply Chain Security
- SBOM
- Trivy
- Threat Modeling
- OWASP API Top 10

## Deliverables

- Security pipeline
- Image scanning
- Dependency scanning

---

# Phase 8 — Stellar Fundamentals

**Duration**

Days 41–46

## Topics

- Accounts
- Assets
- Transactions
- Fees
- Trustlines
- RPC
- Multi-signature
- Stellar CLI

## Mini Projects

- Wallet
- Asset Issuer
- Payments

## Deliverables

Working Stellar client

---

# Phase 9 — Soroban Smart Contracts

**Duration**

Days 47–54

## Topics

- Storage
- Authorization
- Events
- Testing
- Deployments

## Mini Projects

- Counter
- Escrow
- Voting
- Treasury

## Deliverables

Working Soroban applications

---

# Phase 10 — Akawo3 Smart Contract

**Duration**

Days 55–60

## Entities

- Group
- Member
- Cycle
- Contribution
- Payout

## Functions

- create_group()
- join_group()
- contribute()
- start_cycle()
- complete_cycle()
- claim_payout()
- close_group()

## Deliverables

Production-ready smart contract

---

# Phase 11 — Backend Integration

**Duration**

Days 61–64

## Topics

- Stellar RPC
- Transaction Builder
- Wallet Verification
- Retry Logic
- Indexing
- Background Workers

## Deliverables

Backend ↔ Blockchain integration

---

# Phase 12 — Frontend

**Duration**

Days 65–70

## Pages

- Landing
- Dashboard
- Groups
- Wallet
- Profile
- Savings
- Transactions
- Settings

## Topics

- Signals
- Routing
- Forms
- API Integration

## Deliverables

Complete frontend

---

# Phase 13 — NFTs & Reputation

**Duration**

Days 71–73

## NFTs

### Membership NFT

Issued when joining a group.

---

### Completed Cycle NFT

One NFT per successfully completed savings cycle.

Metadata

- Group
- Members
- Total Saved
- Cycle Number
- Timestamp

---

### Soulbound Reputation Badge

Milestones

- First Cycle
- Five Cycles
- Trusted Saver
- Community Leader

## Deliverables

NFT subsystem completed

---

# Phase 14 — Production Engineering

**Duration**

Days 74–77

## Topics

- HTTPS
- Reverse Proxy
- Monitoring
- Tracing
- Metrics
- Backups
- Disaster Recovery
- Health Checks

## Deliverables

Production deployment

---

# Phase 15 — Polish & Portfolio

**Duration**

Days 78–80

## Topics

- Refactoring
- Documentation
- Performance
- Security Review
- Architecture Diagrams
- README
- Demo Recording
- Portfolio Optimization

## Deliverables

- Production-ready repository
- Architecture documentation
- Deployment guide
- Technical write-up
- Demo video

---

# Testing Strategy

## Unit Tests

Business logic

## Integration Tests

API

## Contract Tests

Soroban

## End-to-End Tests

Frontend → Backend → Blockchain

---

# Security Checklist

- JWT Authentication
- Wallet Authentication
- HTTPS
- Secure Headers
- Rate Limiting
- Secrets Management
- Dependency Scanning
- Container Scanning
- SBOM
- Supply Chain Security

---

# CI Pipeline

```

Developer Push

↓

cargo fmt

↓

cargo clippy

↓

cargo test

↓

cargo audit

↓

cargo deny

↓

SQLx migration check

↓

Docker Build

↓

Trivy Scan

↓

SBOM Generation

↓

Integration Tests

↓

Deployment

```

---

# Definition of Done

Every feature must include:

- Business Logic
- Tests
- Documentation
- Logging
- Metrics
- Security Review
- API Documentation
- CI Passing

---

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

It is an opportunity to learn how production-grade fintech systems are designed, built, secured, tested, deployed, and maintained.

The ultimate goal is not merely to complete the application, but to become a **Rust Backend Engineer**, **Blockchain Engineer**, and **Software Engineer** capable of designing and delivering production-ready systems with confidence.