# ADR 0005: Local Data Boundary

## Status

Proposed

## Context

Northworth will eventually handle sensitive household financial data, including imported transactions, account profiles, portfolios, income facts, mortgage details, tax facts, interview answers, assumptions, projections, and audit trails. The app is intended to be local-first and useful offline, with optional self-hosted or online integrations only when explicitly designed.

The project needs a storage boundary before durable private persistence begins. Public source caches and private household data have different risk profiles, and encryption-at-rest choices affect backup, recovery, support, and cross-platform behavior.

## Decision

Northworth will separate public source cache data from user-private financial data.

Public source caches may be stored locally when they include retrieval dates, source metadata, and stale-state behavior. User-private financial data must not be persisted durably until key management, recovery behavior, backup/export behavior, and redaction rules are accepted.

Encryption at rest is a gate for real private financial persistence. If M1 requires private persistence, M1 must include a narrow encrypted-storage implementation. If M1 can stay with import previews, deterministic calculations, and source-backed public caches, full private persistence and encryption hardening may move to M2.

The detailed product boundary is documented in [docs/product/local-data-boundary.md](../product/local-data-boundary.md), with QA cases in [docs/qa/local-data-privacy-cases.md](../qa/local-data-privacy-cases.md).

## Consequences

This slows down durable storage of private financial data, but it prevents the project from accidentally shipping an unsafe local database or misleading backup story. It also lets source-backed calculations continue offline with cached public data while private persistence is still being designed.

Future implementation work must document the database choice, encryption mechanism, OS secret-storage behavior, backup/export behavior, and key-loss recovery language before storing real private data.

## Review

- Owning team: Architecture & Platform
- Review partners: Privacy, Safety & QA; Product & ADLC Ops; Canadian Tax & Accounts; Investment Strategy Engine
- Date: 2026-06-15
- Related issues: #41, #42

