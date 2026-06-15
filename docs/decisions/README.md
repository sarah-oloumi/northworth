# Decision Log

Use this directory for Architecture Decision Records (ADRs) and product decision records that affect Northworth's structure, safety posture, or user-facing behavior.

Create a decision record when a change:

- chooses or rejects a meaningful dependency
- changes storage, privacy, or security behavior
- defines finance/tax calculation boundaries
- creates a new source governance rule
- changes a public workflow, data format, or release policy
- resolves a tradeoff that future contributors would otherwise reopen

Name records with a zero-padded sequence and short slug:

```text
0001-local-first-desktop.md
0002-source-backed-tax-rules.md
```

Use [template.md](./template.md) for new records.

## Records

- [0001: Local-first ADLC](./0001-local-first-adlc.md)
- [0002: Advice Boundaries](./0002-advice-boundaries.md)
- [0003: AI Privacy Boundary](./0003-ai-privacy-boundary.md)
- [0004: Source Governance](./0004-source-governance.md)
- [0005: Local Data Boundary](./0005-local-data-boundary.md)
- [0006: Encryption At Rest](./0006-encryption-at-rest.md)
- [0007: OFX/QFX Import Parser](./0007-ofx-qfx-import-parser.md)
