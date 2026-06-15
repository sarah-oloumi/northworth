# ADR 0006: Encryption At Rest

## Status

Proposed

## Context

Northworth will eventually persist highly sensitive local financial data. The app is local-first and does not have a hosted account, server-side reset, or cloud recovery service. Durable persistence therefore needs an encryption and recovery model before real private data is saved.

The project already separates public source cache data from private household data in [ADR 0005](./0005-local-data-boundary.md). The next decision is how to encrypt the private store and protect local keys.

## Decision

Northworth should use encrypted private persistence when real user-private financial data is durably stored.

The recommended MVP design is a random per-profile data-encryption key protected by OS secret storage. Private financial payloads should be encrypted before being written to disk. Public source cache data should remain in a separate storage boundary and may be stored without private-data encryption when it contains only public source metadata, retrieval timestamps, and stale-state fields.

Passphrase-derived recovery or portable encrypted exports should be treated as an M2 hardening feature unless M1 cannot meet user needs without cross-device private-data portability.

No private persistence implementation should begin until exact crates, key-loss behavior, backup/export behavior, and Linux keychain behavior are accepted.

Detailed evaluation is documented in [docs/product/encryption-at-rest-spike.md](../product/encryption-at-rest-spike.md).

## Consequences

This keeps import previews and source-backed calculations moving while preventing accidental unencrypted storage of private financial records.

The recommended path gives a smoother desktop UX than passphrase-first storage, but it creates device and OS-account recovery tradeoffs. The app must explain that encrypted local data may be unrecoverable if the local keychain secret is lost.

Implementation will need focused tests for key unavailable, key lost, backup restore, migration failure, redacted logs, and synthetic-only fixtures.

## Review

- Owning team: Architecture & Platform
- Review partners: Privacy, Safety & QA; Product & ADLC Ops
- Date: 2026-06-15
- Related issues: #41, #42

