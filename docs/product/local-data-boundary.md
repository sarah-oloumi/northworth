# Local Data Boundary

Northworth is a local-first finance app. It may eventually store imported bank files, household profiles, portfolios, assumptions, projections, source caches, and audit trails. This document defines what can be stored, how private data is separated from public source data, and what must be true before private data persistence ships.

## Default Position

The default storage posture is:

- no hosted Northworth account
- no telemetry or analytics
- no private financial data in the public repository
- no private financial data in fixtures, screenshots, logs, crash reports, commits, or pull requests
- no silent network sync
- no remote AI dependency for core workflows
- no authoritative calculations from AI output
- source-backed calculations remain usable offline when their source data is cached

Until encrypted local persistence is designed and reviewed, private financial data should remain in memory, in user-selected import previews, or in explicit user-controlled files.

## Data Classes

Northworth treats local data as one of four classes.

| Class | Examples | Default handling |
| --- | --- | --- |
| Public source data | CRA source metadata, tax constants, official source URLs, Bank of Canada source metadata, calculation formula versions | May be cached locally with source timestamps and stale-state metadata. |
| User-private financial data | Imported transactions, account balances, mortgage details, income facts, holdings, cost bases, employer details, tax facts, dependents, interview answers, notes | Must not leave the device by default. Must not be persisted without an accepted storage and encryption decision. |
| Local secrets | API keys, encryption keys, provider tokens, local model credentials | Must be stored through OS-provided secret storage or an accepted equivalent. Must not be exported or logged by default. |
| Derived local planning data | Budgets, categories, projections, scenario assumptions, audit trails, risk profiles, strategy outputs | Treat as private when derived from private inputs. Store with calculation and source metadata when persistence exists. |

Synthetic sample data is allowed in the repository when it is clearly fictional and cannot be mistaken for a real household.

## Storage Boundary

Private data and public source data must use separate storage boundaries.

Public source cache:

- may be refreshed from official or reviewed providers
- must include retrieval date, source version or effective date, and stale-after behavior
- may be cleared without deleting user-private financial records
- must not include imported transaction descriptions, account identifiers, names, addresses, or notes

User-private store:

- must be local by default
- must be encrypted at rest before real private persistence ships, unless a later accepted decision explicitly narrows the risk
- must use migrations that preserve data integrity and can fail safely
- must support backup and export rules before user-private persistence is considered complete
- must keep audit trails local and redacted when shown in diagnostics

Temporary import previews:

- may parse CSV, OFX, and QFX files locally
- must not copy source files into the repository
- must not log raw transaction descriptions by default
- must make it clear when data is only previewed and not saved

## Encryption at Rest Recommendation

Encryption at rest should be treated as a gate for any durable storage of real private financial data.

Recommended M1 path:

- Allow deterministic calculations, import previews, budget previews, and source-backed public caches.
- Do not persist real imported transactions, household profiles, portfolios, or scenario history until key management and recovery behavior are accepted.
- If M1 requires private persistence, include encryption at rest in M1 with a narrow first version.
- If M1 can remain preview/in-memory for private data, move full private persistence and encryption hardening into M2.

Candidate approaches for the implementation spike:

| Approach | Benefit | Tradeoff |
| --- | --- | --- |
| OS keychain protects a local database key | Uses platform secret storage and keeps the user experience simple. | Keychain behavior differs across macOS, Windows, and Linux; recovery UX must be designed. |
| User passphrase protects the database key | Works across platforms and makes backups more portable. | Forgotten passphrases cannot be reset by a hosted service because Northworth is local-only. |
| Per-file encrypted exports | Gives users explicit portable backups. | Requires careful copy, restore, and key-loss language. |
| Unencrypted private database | Simplest implementation. | Not acceptable for real private financial data without a separate accepted risk decision. |

The first implementation should prefer boring, reviewable storage over clever abstractions. Database choice, encryption mechanism, and key storage should be documented in a future ADR before code persists private data.

## Key Loss and Recovery

Northworth cannot offer hosted account recovery if the app is local-only.

The product must make this tradeoff visible:

- If the encryption key or passphrase is lost, encrypted local data may be unrecoverable.
- Backups must clearly state whether they include private data, source caches, secrets, or all local data.
- Exports must be explicit user actions.
- API keys and encryption keys must not be included in normal diagnostic bundles.
- Recovery language must avoid implying that Northworth can reset a lost key remotely.

## Logs and Diagnostics

Logs must be safe to share by default.

Logs and diagnostics must not include:

- names, addresses, employers, or account numbers
- raw imported transaction descriptions
- file paths that reveal private household details
- account balances, salaries, mortgage amounts, holdings, cost bases, or net worth
- API keys, provider tokens, encryption keys, or local model credentials
- full prompt or model output text from private scenarios

Diagnostics may include:

- app version
- operating system family
- calculation engine version
- source metadata ids and stale/current status
- error codes
- redacted import parser statistics

## Offline Behavior

Northworth must remain useful offline:

- imports still parse locally
- local data remains readable when persistence exists
- source-backed calculations run with cached source data and visible stale-state warnings
- interviews run from local questions and templates
- explanations degrade to deterministic copy when remote AI or local models are unavailable
- remote AI failure never blocks imports, budgets, calculations, or scenario review

## Security Review Checklist

Before a feature persists, exports, logs, syncs, or sends private data, reviewers should confirm:

- Does the feature work without a network connection?
- Is private data separated from public source cache data?
- Is real private persistence encrypted at rest or explicitly blocked?
- Are key storage and key-loss behaviors documented?
- Are backups and exports explicit user actions?
- Are logs and diagnostics redacted by default?
- Are tests, fixtures, screenshots, and PR examples synthetic?
- Does the feature preserve source-backed deterministic calculations?
- Does stale or missing source data remain visible to the user?

