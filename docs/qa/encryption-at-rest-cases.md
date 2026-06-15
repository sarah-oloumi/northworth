# Encryption At Rest QA Cases

Use these cases when reviewing local private persistence, backups, restores, exports, diagnostics, and migrations.

| Case | Risk | Failing behavior | Acceptable behavior |
| --- | --- | --- | --- |
| First private save | Private data is written before encryption is ready | Imported transactions are saved in plain local storage. | Save is blocked or encrypted private persistence is used. |
| Key unavailable | App crashes or loses data | OS keychain is unavailable and the app overwrites or deletes encrypted records. | App fails closed, explains the local key problem, and preserves existing data. |
| Key loss | Recovery is overpromised | App says Northworth can reset the local encryption key. | App explains that local encrypted data may be unrecoverable without the key or recovery secret. |
| Backup export | Secrets leak | Export includes API keys, provider tokens, or raw private data without explicit encryption. | Private export is explicit, encrypted, and excludes secrets by default. |
| Restore | Existing data is overwritten | Restore replaces a local profile without confirmation or validation. | Restore validates schema and uses an explicit conflict-safe flow. |
| Logs | Private data leaks | Logs include raw transactions, balances, holdings, or key material. | Logs contain redacted error codes and synthetic test data only. |
| Public source cache | Boundaries blur | Source cache includes imported transaction text or private account identifiers. | Source cache stores only public source data and freshness metadata. |
| Migration | Partial write corrupts data | Failed migration leaves neither the old nor new encrypted store readable. | Migration is transactional or preserves a recoverable prior state. |

## Checklist

- Is private persistence encrypted or explicitly blocked?
- Is keychain/passphrase behavior visible and testable?
- Does the feature fail closed when key material is unavailable?
- Are backups and exports explicit user actions?
- Are secrets excluded from diagnostics and normal exports?
- Are public source caches separate from private household data?
- Are all fixtures and examples synthetic?

