# ADR 0002: Advice and Recommendation Boundaries

## Status

Accepted

## Context

Northworth will eventually compare tax, investment, debt, real estate, household, and registered-account scenarios. These outputs can influence high-impact financial decisions even when the app does not execute trades or file taxes.

The project needs product-language and review boundaries before implementation begins so scenario engines do not accidentally sound like regulated financial, legal, tax, accounting, insurance, mortgage, or securities advice.

## Decision

Northworth will treat recommendation-like output as educational scenario planning. High-impact outputs must show assumptions, sources, source dates, missing facts, stale-data warnings, tradeoffs, and professional-review flags where relevant.

The app must not execute trades, claim suitability, promise outcomes, or hide uncertainty. Product language must follow [docs/product/advice-boundaries.md](../product/advice-boundaries.md), and QA copy review must use [docs/qa/misleading-language-cases.md](../qa/misleading-language-cases.md).

## Consequences

This limits the app's tone and makes some outputs more cautious, but it protects users from overconfident guidance and gives future implementation work a clear safety contract.

## Review

- Owning team: Product & ADLC Ops
- Review partners: Privacy, Safety & QA; Canadian Tax & Accounts; Investment Strategy Engine
- Date: 2026-06-14
- Related issues: #22

