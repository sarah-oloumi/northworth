# Encryption At Rest Spike

This spike evaluates how Northworth should protect private financial data when durable local persistence is introduced.

Northworth is local-first, so there is no hosted account recovery, server-side reset, or remote vault by default. Encryption choices must be understandable to users and boring enough for contributors to maintain.

## Scope

Private data that requires protection:

- imported CSV, OFX, and QFX transactions
- account profiles, balances, and institution labels
- portfolios, holdings, tax lots, cost bases, and ACB records
- income, tax, dependent, Indigenous tax-treatment, foreign-property, and household facts
- assumptions, projections, risk interviews, notes, and scenario audit trails
- local API keys, provider tokens, local model credentials, and encryption keys

Public source cache data may be stored separately when it contains only public facts, source metadata, retrieval dates, and stale-state fields.

## Discovery Notes

On 2026-06-15, `cargo search` showed current candidates in the Rust ecosystem:

- `keyring` for OS credential storage
- `rusqlite` for SQLite access
- `chacha20poly1305` and related AEAD crates for authenticated encryption
- `argon2` and related crates for passphrase-derived keys

These searches confirm that viable crate families exist. They do not select final dependencies. Before implementation, the exact crates must be verified for maintenance status, audit posture, platform support, licenses, and Tauri 2 compatibility.

## Evaluated Patterns

| Pattern | How it works | Benefits | Risks |
| --- | --- | --- | --- |
| Keychain-wrapped data key with encrypted private records | Generate a random local data-encryption key, store or wrap it with OS secret storage, and encrypt private payloads before writing them to a local database. | Good desktop UX, no user passphrase by default, keeps private values encrypted at rest, fits local-first use. | OS keychain behavior differs across macOS, Windows, and Linux; record-level encryption limits querying unless indexes are carefully designed. |
| SQLCipher or encrypted SQLite database | Store private data in an encrypted SQLite database. Unlock locally with a key protected by OS secret storage or passphrase. | Encrypts the whole database file and preserves normal relational query patterns. | Native linking and cross-platform packaging may be more complex; dependency and migration behavior need careful review. |
| Passphrase-derived encrypted file or database key | User passphrase derives or unlocks the local encryption key. | Portable across devices and backups without relying only on OS keychain. | Forgotten passphrases cannot be reset by Northworth; adds unlock UX and support complexity. |
| Unencrypted private SQLite/database | Store private data directly in a local database. | Fastest implementation. | Rejected for real private financial data unless a future accepted risk decision explicitly narrows the scope. |

## Recommended MVP Direction

If M1 requires durable private persistence, implement a narrow encrypted local store:

- Use a random per-profile data-encryption key.
- Protect the data key with OS secret storage.
- Encrypt user-private financial payloads before writing them to disk.
- Store public source cache data separately from private household data.
- Do not sync, upload, or log private data.
- Keep imports preview-only until this encrypted store exists.

If M1 can remain preview/in-memory for private data, defer encrypted private persistence to M2, but keep source caches and deterministic calculations moving.

Recommended default:

- **M1**: keychain-wrapped local data key for encrypted private persistence only if private persistence is necessary.
- **M2**: optional passphrase/recovery export flow for portability and users who do not want to rely only on OS keychain behavior.

## Key Management Requirements

Before implementation begins, the project must decide:

- whether each local profile has one data key or separate keys by data domain
- whether the key is stored directly in OS keychain or wrapped by a keychain-protected secret
- whether Linux Secret Service availability is required for MVP or whether Linux private persistence starts behind a capability warning
- how the app behaves when the keychain is locked, unavailable, corrupted, or migrated
- how backups behave when restored to a different device or OS account

## Recovery Behavior

Northworth cannot remotely reset a local key.

Required product language:

- If the OS keychain secret is lost, encrypted local data may be unrecoverable.
- A normal diagnostic bundle must not include the encryption key, API keys, or raw private data.
- Backup/export flows must state whether the export can be restored on another device.
- Portable encrypted exports require either a passphrase or another explicit recovery secret.
- The app must not imply that Northworth contributors can recover encrypted user data.

## Backup And Export Implications

MVP backups should be conservative:

- Source cache export may be separate from private-data export.
- Private backup/export must be explicit and encrypted.
- Secrets should be excluded by default.
- Export files must include schema version, app version, encryption mode, and creation timestamp.
- Restore must validate the schema and fail without overwriting existing local data.

## Implementation Gate

Do not implement durable private persistence until all are true:

- an encryption ADR is accepted
- exact crates are selected and reviewed
- key-loss and restore behavior are agreed
- private and public storage boundaries are represented in code
- logs and diagnostics are redacted
- tests use synthetic data only

