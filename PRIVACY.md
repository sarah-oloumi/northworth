# Privacy Policy For Development

This repository is designed for a public codebase and a private local app.

## Never Commit

- Real names, addresses, employers, salaries, mortgage balances, tax slips, account balances, or net worth.
- Exported user plans.
- Browser localStorage dumps.
- Private screenshots or screen recordings.
- API keys, tokens, cookies, credentials, or account identifiers.
- Documents from banks, brokerages, insurers, employers, CRA, or mortgage providers.

## Safe To Commit

- Fictional demo households.
- Generic examples with rounded, invented values.
- Public tax rules with source references.
- Code, tests, and documentation that do not reveal private finances.

## Market Data Boundary

The app may fetch public market prices, but private household data must not be sent to market data providers. The provider layer should request symbols or public identifiers only.

## Local Data Boundary

User plans and imported financial data should be stored locally by default. Durable private persistence requires an accepted storage, encryption, backup, and recovery design before implementation. Cloud sync, analytics, or account features require an explicit design review before implementation.

See [docs/product/local-data-boundary.md](docs/product/local-data-boundary.md) before adding local persistence, backups, exports, diagnostics, or audit trails.

## AI Privacy Boundary

Remote AI is disabled by default. The app may support offline local models and explicit online provider mode with user-supplied API keys. Private financial facts must not be sent to an AI service unless the user explicitly enables that feature, reviews the selected data, and confirms the send.

See [docs/product/ai-privacy-boundary.md](docs/product/ai-privacy-boundary.md) before adding interviews, explanations, categorization helpers, or assistant-like behavior.
