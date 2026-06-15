# Northworth

A private, local-first Canadian household finance app for wealth building, tax planning, and investment strategy.

This repository is intended to be public-safe. It must never contain real household balances, addresses, mortgage documents, account numbers, tax slips, exported plans, screenshots of private plans, or other personal financial details.

## Product Direction

- Model monthly household cash flow.
- Track income from salary, bonus, equity compensation, and other sources.
- Support Ontario and Canadian tax-aware planning.
- Compare RRSP, TFSA, DPSP, FHSA, group plans, and non-registered investment strategies.
- Model real estate, mortgages, insurance, vehicles, debt, and major future expenses.
- Explore legal tax-efficiency strategies.
- Pull public market data through a provider layer without sending private household data to third parties.
- Ship with fictional demo data only.

## Privacy Principles

- Real user data stays local.
- Demo data must be fictional.
- No analytics by default.
- No account signup required.
- No private financial data in git.
- No personal data in tests, docs, screenshots, or fixtures.

See [PRIVACY.md](./PRIVACY.md) before adding features.

## Development

This repo is scaffolded but intentionally not implemented yet.

```bash
cargo run -p northworth
```

Northworth is a desktop app scaffolded with Rust + Tauri and plain static HTML/CSS. It does not require Node or npm.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for commit, pull request, and review standards. See [docs/release-process.md](./docs/release-process.md) for versioning and tag policy.

See [docs/design/product-brief.md](./docs/design/product-brief.md) for the initial product and UX direction.

See [docs/product/advice-boundaries.md](./docs/product/advice-boundaries.md) before adding tax, investment, debt, insurance, real estate, or recommendation-like features.

See [docs/product/ai-privacy-boundary.md](./docs/product/ai-privacy-boundary.md) before adding interviews, explanations, categorization helpers, or assistant-like behavior.

See [docs/product/local-data-boundary.md](./docs/product/local-data-boundary.md) and [docs/product/encryption-at-rest-spike.md](./docs/product/encryption-at-rest-spike.md) before adding durable private persistence, backups, restores, exports, or diagnostics.

See [docs/product/source-governance.md](./docs/product/source-governance.md) before adding tax rules, account limits, market data, benefits, projections, or formula-backed financial calculations.

## Status

Early setup. The tax, market data, and planning engines have not been built yet.

## License

Northworth is licensed under the GNU Affero General Public License v3.0 or later. See [LICENSE](./LICENSE).
