# ADR 0003: AI Privacy Boundary

## Status

Accepted

## Context

Northworth may eventually include interview, explanation, categorization, and scenario-summary features that feel AI-assisted. The app will also handle highly sensitive household financial data. Remote AI calls, prompt logs, telemetry, API key handling, and overly broad consent could expose private financial facts.

The project needs a privacy boundary before any AI-like feature is implemented.

## Decision

Northworth will be local/private by default. AI-assisted features may support disabled/deterministic mode, offline local model mode, and online provider mode with user-supplied API keys such as OpenAI API keys.

Remote AI is disabled by default and blocked until a separate decision record, data-flow review, explicit user opt-in, visible data selection, cancellation before sending, local API-key control, and prompt/output logging rules exist. Offline local model mode must not send private financial data to remote services and must degrade gracefully when unavailable.

AI output must follow [docs/product/advice-boundaries.md](../product/advice-boundaries.md). AI must not invent source-backed rules, perform authoritative calculations, override formula results, or provide trade/tax/legal instructions.

The detailed policy is documented in [docs/product/ai-privacy-boundary.md](../product/ai-privacy-boundary.md), with QA cases in [docs/qa/ai-privacy-cases.md](../qa/ai-privacy-cases.md).

## Consequences

This keeps the default app private and usable offline, while leaving room for users who want either local LLMs or explicit online provider integrations. Remote AI convenience features require explicit design and review work before implementation.

## Review

- Owning team: Privacy, Safety & QA
- Review partners: Product & ADLC Ops; Architecture & Platform; UX & Interaction
- Date: 2026-06-14
- Related issues: #64
