# ADR 0007: OFX/QFX Import Parser

## Status

Accepted

## Context

Northworth needs a local OFX/QFX import path for chequing, savings, credit card, and brokerage cash transactions. Imported statement files can contain private transaction descriptions, account identifiers, and balances, so parsing must happen locally and must not require credentials or a network service.

The MVP import workflow is preview-only until encrypted private persistence is accepted. That means the first parser should normalize transactions for review, show safe validation errors, and keep the implementation small enough to audit.

## Decision

Northworth will use an internal Rust OFX/QFX transaction-preview parser for M1.

The parser reads `STMTTRN` blocks from OFX SGML-style exports and QFX files, extracts `DTPOSTED`, `TRNAMT`, and `NAME` or `MEMO`, then normalizes valid rows into the same `TransactionRecord` model used by CSV imports. Imported rows remain `PendingReview` and are not persisted by the import command.

The parser intentionally returns structured field errors instead of logging raw file contents or raw invalid values. Synthetic OFX and QFX fixtures live under `examples/imports/`.

This is an MVP parser, not a full OFX financial-data engine. Expanding to securities transactions, account metadata preservation, balances, FITID-based duplicate detection, broker-specific QFX variants, or full XML/SGML compatibility should happen through follow-up issues after real-world fixture requirements are defined with synthetic samples.

## Consequences

The app can preview common local OFX/QFX exports without adding a network dependency or a large parser dependency before storage and privacy boundaries settle.

The tradeoff is narrower format coverage. Unsupported tags or malformed files may produce safe parser errors instead of partial imports. Future brokerage import work should revisit whether the internal parser remains enough or whether a dedicated Rust OFX parser is justified.

## Review

- Owning team: Data Ingestion
- Review partners: Architecture & Platform; Privacy, Safety & QA
- Date: 2026-06-15
- Related issues: #11, #24, #54
