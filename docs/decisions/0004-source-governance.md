# ADR 0004: Source Governance for Financial Rules

## Status

Accepted

## Context

Northworth will use Canadian tax rules, account limits, benefits, FX data, market data, issuer facts, and planning assumptions. These values change over time and can materially affect high-impact outputs. The app must not rely on unsourced constants, stale model memory, or AI-generated facts.

## Decision

Northworth will require source metadata for tax, account, benefit, market-data, and formula-backed rules. Source-backed data must include jurisdiction, tax year or effective date, source URL, publisher, retrieved date, last reviewed date, status, and review ownership.

Authoritative calculations must use deterministic formula code with typed inputs, source-backed constants, visible assumptions, and synthetic expected-result tests. AI may explain results, but it must not provide calculation truth.

The detailed process is documented in [docs/product/source-governance.md](../product/source-governance.md), with QA cases in [docs/qa/source-governance-cases.md](../qa/source-governance-cases.md).

## Consequences

This adds work to every finance/tax feature, but it makes source freshness, review responsibility, and stale-data behavior explicit. It also gives future automation a concrete metadata shape to validate.

## Review

- Owning team: Canadian Tax & Accounts
- Review partners: Privacy, Safety & QA; Architecture & Platform; Investment Strategy Engine
- Date: 2026-06-14
- Related issues: #65

