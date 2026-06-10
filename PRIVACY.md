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

User plans should be stored locally by default, such as browser storage or user-controlled local files. Cloud sync, analytics, or account features require an explicit design review before implementation.

