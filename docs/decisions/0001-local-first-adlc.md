# ADR 0001: Local-First ADLC Governance

## Status

Accepted

## Context

Northworth is a public repository for a local-first finance app. The app will eventually handle sensitive household finance, Canadian tax, investing, debt, real estate, and planning scenarios. Mistakes can cause privacy leaks, misleading recommendations, or unsupported tax assumptions.

The project needs a lightweight operating model before implementation begins so future work has clear ownership, review partners, source expectations, and lesson-learned loops.

## Decision

Northworth will use an ADLC team model documented in [docs/adlc/roles.md](../adlc/roles.md).

Issues and pull requests should identify the relevant team labels. Finance, tax, investment, privacy, and architecture changes require review from the owning team and the listed review partners. Meaningful pull requests and epic closeouts should include lessons learned.

## Consequences

This adds a little process before coding, but gives the project a safer default for sensitive financial features. It also makes backlog ownership visible in GitHub labels and keeps review expectations consistent.

## Review

- Owning team: Product & ADLC Ops
- Review partners: Architecture & Platform; Privacy, Safety & QA
- Date: 2026-06-14
- Related issues: #6

