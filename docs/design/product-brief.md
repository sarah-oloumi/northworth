# Northworth Product Brief

Northworth is a local-first Canadian household finance planner for wealth building, cash flow, tax awareness, and investment strategy.

The app should feel like a serious personal finance tool: calm, plain-spoken, fast, and trustworthy. The design goal is not novelty. The goal is that a user can understand their household financial position, test decisions, and inspect the rules behind the calculations without wondering where their data went.

## Product Principles

- **Local-only by default**: the app runs on the user's machine. No account, telemetry, hosted sync, or cloud storage is required.
- **Self-hostable by choice**: advanced users may run it on their own network or behind a VPN, but the product must not require a public service.
- **Transparent calculations**: every important output should be traceable to inputs, assumptions, and source references.
- **Public-safe repository**: fixtures, screenshots, examples, and docs must use fictional data.
- **Useful before clever**: prefer familiar controls, predictable navigation, and clear tables over custom interactions.
- **Canada-first, Ontario-first**: model Canada/Ontario rules first, with room for future provinces.
- **Strategy, not tax evasion**: the app can compare legal tax-efficient choices, but must not suggest hiding income or misrepresenting facts.

## Primary User Jobs

- See household net worth and monthly cash flow at a glance.
- Understand where money is going each month.
- Compare investment account choices: RRSP, TFSA, DPSP, FHSA, group plans, and non-registered accounts.
- Understand the tax effect of salary, bonus, RSUs, capital gains, dividends, interest, and deductions.
- Model property, mortgages, renewals, insurance, maintenance, and carrying costs.
- Compare strategies such as mortgage prepayment vs investing, RRSP vs TFSA, selling RSUs vs holding, and taxable asset location.
- Inspect CRA/Ontario rule references and see when constants were last reviewed.
- Export or import local data without sending it to a hosted service.

## Navigation Model

Use a persistent primary navigation with tabs or a sidebar depending on viewport width. The navigation should be stable across the app so users build muscle memory.

Initial tabs:

- **Dashboard**: household overview, alerts, next decisions, and key metrics.
- **Cash Flow**: monthly income, recurring expenses, debt payments, surplus/shortfall, and annualized totals.
- **Investments**: accounts, holdings, allocation, contribution room, fees, and public market data.
- **Strategy**: side-by-side what-if comparisons and recommended next actions with assumptions shown.
- **Property**: primary home, secondary property, mortgages, renewal dates, equity, insurance, and carrying costs.
- **Tax**: projected tax, marginal rates, deductions, credits, RSUs, capital gains, dividends, and registered-account effects.
- **References**: CRA/Ontario sources, tax-year constants, market-data source notes, and update history.
- **Settings**: local data storage, import/export, demo data, market data provider configuration, and privacy controls.

## First Implementation Slice

Build the **References** tab first.

Why:

- It has low privacy risk.
- It forces the project to define source-backed data conventions early.
- It gives future tax/account calculations a transparent foundation.
- It creates a place to show last-reviewed dates, source links, and uncertainty.

The first References version should include:

- Static source cards for CRA, Ontario, and market-data provider notes.
- A table of planned rule domains: tax brackets, RRSP, TFSA, DPSP, FHSA, capital gains, dividends, mortgage assumptions, and market data.
- Fields for `jurisdiction`, `taxYear`, `sourceURL`, `lastReviewed`, `status`, and `notes`.
- No private user data.

## Dashboard Direction

The Dashboard should answer:

- What is the current household picture?
- What changed since the last review?
- What decisions need attention soon?
- Which assumptions drive the result?

Preferred sections:

- **Net worth**: assets, liabilities, and trend.
- **Monthly flow**: income, expenses, savings rate, and shortfall risk.
- **Tax outlook**: estimated tax owing/refund, marginal rates, and deduction opportunities.
- **Investment posture**: allocation, registered room, concentration risk, and pending RSU events.
- **Property position**: mortgage balances, equity, renewal dates, and carrying costs.
- **Next decisions**: prioritized list with why it matters.

## UX Rules

- Use plain language first, with source links for exact legal/tax wording.
- Show assumptions beside outputs, not buried in settings.
- Prefer inline expansion over modals.
- Use tables for comparable financial data.
- Use charts when they reveal a trend or tradeoff, not as decoration.
- Let users switch between monthly and annual views where the difference matters.
- Make empty states instructional and demo-data friendly.
- Keep all destructive actions explicit and reversible where practical.
- Never rely on color alone to communicate financial status.

## Visual Direction

Northworth should use a restrained red and white palette:

- White and near-white surfaces.
- Dark neutral text.
- Canadian red for brand, active navigation, primary actions, focus, and important warnings.
- Muted green only for positive financial state.
- Amber only for caution or assumptions that need review.
- Blue only for informational links or source references if needed.

Avoid:

- Decorative gradients.
- Oversized marketing-style hero sections.
- Card grids for everything.
- Nested cards.
- Display fonts in labels, buttons, tables, or forms.
- Red/green-only status encoding.

The UI should feel closer to a polished professional dashboard than a banking ad.

## Layout Direction

- Use a two-column app shell on desktop: primary navigation plus content.
- Collapse navigation structurally on small screens.
- Use consistent spacing tokens.
- Keep repeated controls in predictable locations.
- Use dense but readable tables for financial details.
- Keep cards for distinct repeated items, summaries, or grouped decisions.
- Use full-width sections and dividers for page structure instead of stacking cards inside cards.

## Local Data Model Boundaries

Northworth can have three data categories:

- **Application data**: source-backed constants and reference metadata committed to the repo.
- **Demo data**: fictional household examples committed to the repo.
- **Private user data**: local-only data ignored by git and never sent to hosted services.

Market-data requests must not include private household data. Provider calls should request public symbols or identifiers only.

## Design References

The repository vendors Impeccable as a pinned design reference. For Northworth, apply its product-UI guidance in a restrained way:

- familiar controls over novelty
- product density over marketing spaciousness
- fixed type scale over fluid display type
- semantic color over decoration
- consistent component vocabulary across screens

