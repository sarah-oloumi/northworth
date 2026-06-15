# Local Data Privacy QA Cases

Use these cases when reviewing imports, local persistence, source caches, logs, exports, backups, diagnostics, and scenario audit trails.

## Test Matrix

| Case | Risk | Failing behavior | Acceptable behavior |
| --- | --- | --- | --- |
| Import preview | Imported bank data is silently saved | CSV, OFX, or QFX data is written to a durable private database before storage rules exist. | Import data is parsed locally and shown as preview-only until private persistence is accepted. |
| Public source cache | Private data mixes with source data | Transaction descriptions or balances are stored in the same cache as CRA or Bank of Canada source metadata. | Public source cache contains only public source data, metadata, retrieval dates, and stale-state fields. |
| Logs | Sensitive data appears in logs | Logs include names, addresses, file paths, transaction descriptions, balances, salaries, holdings, or mortgage amounts. | Logs include only redacted diagnostics, error codes, parser counts, and source metadata ids. |
| Local database | Unencrypted private persistence ships | Imported transactions or household profiles are stored unencrypted by default. | Private persistence is blocked until encryption, key storage, recovery, and backup behavior are accepted. |
| Backup | User misunderstands backup contents | Backup exports silently include API keys, encryption keys, or raw private data. | Backup/export flows explicitly state contents and exclude secrets by default. |
| Key loss | Recovery language overpromises | App says Northworth can reset a lost local encryption key. | App explains local-only recovery limits and warns that lost keys may make encrypted data unrecoverable. |
| Diagnostics | Shareable bundle leaks data | Diagnostic export includes private scenario notes or imported transaction rows. | Diagnostic export is redacted by default and uses synthetic or aggregate details. |
| Offline use | Core workflow requires network | Imported data or saved local plans cannot be viewed offline. | Local workflows remain available offline with stale-source warnings where needed. |
| Audit trail | Explainability leaks private facts | Audit trail shown in diagnostics includes exact salaries, account balances, or transaction text. | Audit trail remains local and redacts private values outside the main app context. |
| Repository hygiene | Real data lands in git | Fixtures, screenshots, PR bodies, or examples contain real household details. | Repository content uses clearly fictional synthetic data only. |

## Review Checklist

- Is the feature local-first and usable offline?
- Is private data separated from public source cache data?
- Is durable private persistence either encrypted at rest or explicitly blocked?
- Are key storage, key loss, and recovery tradeoffs visible?
- Are backups and exports explicit user actions?
- Are logs and diagnostics redacted by default?
- Are source caches timestamped and stale-state aware?
- Are tests, fixtures, screenshots, and PR examples synthetic?
- Does the feature avoid remote AI as a dependency for core workflows?
- Are deterministic calculations preserved as the source of truth?

