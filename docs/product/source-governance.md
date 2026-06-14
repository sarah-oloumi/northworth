# Source Governance

Northworth calculations must be deterministic, formula-based, and traceable to source metadata. This document defines how tax rules, account limits, benefits, market data fields, issuer facts, and planning assumptions are sourced, reviewed, versioned, and marked stale.

Source governance exists because financial rules change. The app must be honest when a rule is current, stale, unknown, superseded, estimated, or not yet implemented.

## Principles

- Prefer official primary sources.
- Store source metadata with every rule, constant, formula, and externally refreshed dataset.
- Version rules by jurisdiction and effective date or tax year.
- Treat unknown and stale data as visible product states, not internal errors.
- Never use AI-generated facts as source truth.
- Never use formulas without reviewable inputs, constants, assumptions, and expected-result tests.
- Keep private household data separate from public source data.

## Source Priority

Use the highest available source tier.

1. Official law, regulation, government, regulator, or central-bank source.
2. Official CRA, provincial/territorial, Bank of Canada, issuer, exchange, or regulator guidance.
3. Official provider API documentation for market-data fields.
4. Secondary references only for discovery, never as calculation authority.

If a secondary source finds a rule, the implementation must trace the calculation back to a primary or official source before it can be treated as current.

## Source Metadata Schema

Every source-backed rule or dataset must include:

```yaml
id: cra_federal_tax_brackets_2026
domain: tax
jurisdiction: CA
tax_year: 2026
effective_from: 2026-01-01
effective_to: 2026-12-31
source_url: https://www.canada.ca/...
source_title: Federal tax rates
source_publisher: Canada Revenue Agency
source_retrieved_at: 2026-06-14
last_reviewed_at: 2026-06-14
reviewed_by_team: team: canadian-tax
status: current
confidence: high
formula_owner: team: canadian-tax
qa_owner: team: privacy-safety
notes: Uses official CRA tax package/source for the tax year.
```

Optional fields:

```yaml
province_or_territory: ON
market_data_provider: Bank of Canada
refresh_cadence: monthly
stale_after_days: 90
supersedes: cra_federal_tax_brackets_2025
professional_review_required: false
```

## Status Values

- `current`: reviewed against an official source for the relevant effective period.
- `stale`: the review date, refresh cadence, or source effective period has expired.
- `unknown`: the source is missing, ambiguous, unavailable, or not reviewed.
- `superseded`: a newer rule or dataset replaces this one.
- `estimated`: a placeholder assumption that must not drive tax-sensitive optimization without warning.

## Stale Data Behavior

The app may display stale or unknown data, but it must not hide the state.

When data is stale or unknown:

- show a visible warning
- show the last reviewed or refreshed date when available
- show the source URL when available
- avoid ranking tax-sensitive or investment-sensitive options as "best"
- allow the user to continue only as an educational scenario
- require review before release if the stale data affects a high-impact output

## Review Cadence

Minimum review cadence:

- Tax brackets, credits, deductions, contribution limits, and benefit thresholds: annually per tax year, plus review when official updates are published.
- Provincial/territorial rules: annually per tax year, plus review when the jurisdiction publishes updates.
- Bank of Canada FX data: refresh by user-selected cadence and preserve the timestamp used in calculations.
- Market prices and issuer facts: refresh by user-selected cadence and show provider/source timestamps.
- Advice-boundary, AI-boundary, and source-governance docs: review before features that depend on them ship.

## Calculation Rules

Formula code must include:

- typed inputs
- explicit formulas or method names
- source-backed constants
- visible assumptions
- expected-result tests with synthetic data
- source metadata references

Formula code must not:

- use AI output as an authoritative calculation
- use unsourced constants
- silently coerce stale or unknown values into current values
- mix tax years without an explicit scenario model
- hide FX dates, rates, or assumptions

## Human Review Gates

Require Canadian Tax & Accounts review for:

- tax brackets, rates, credits, deductions, and benefit calculations
- registered and non-registered account rules
- trusts, Indigenous tax-treatment, foreign property, T1135 awareness, and attribution-rule logic
- provincial/territorial calculation changes

Require Investment Strategy Engine review for:

- projection formulas
- portfolio cost comparison
- risk/risk-capacity models
- asset location and rebalancing logic
- ACB/tax-lot formulas

Require Privacy, Safety & QA review for:

- stale-data warnings
- professional-review flags
- test fixtures and expected results
- privacy-sensitive source handling
- any high-impact output

Require Architecture & Platform review for:

- source metadata schema changes
- data storage layout
- provider abstraction
- update workflow automation

## Update Workflow

1. Identify the rule, dataset, or assumption being added or changed.
2. Capture the official source URL, publisher, effective date, tax year, and retrieved date.
3. Add or update source metadata.
4. Implement or update deterministic formula code.
5. Add synthetic expected-result tests.
6. Add stale/unknown behavior where applicable.
7. Record review teams in the PR.
8. Include source links and review notes in the PR body.

## Official Source Catalog Seeds

Start with these official source families:

- CRA personal income tax packages: https://www.canada.ca/en/revenue-agency/services/forms-publications/tax-packages-years.html
- CRA current T1 package by province/territory: https://www.canada.ca/en/revenue-agency/services/forms-publications/tax-packages-years/general-income-tax-benefit-package.html
- CRA forms and publications: https://www.canada.ca/en/revenue-agency/services/forms-publications.html
- CRA individuals and income tax: https://www.canada.ca/en/revenue-agency/services/tax/individuals.html
- Bank of Canada daily exchange rates: https://www.bankofcanada.ca/rates/exchange/daily-exchange-rates/
- Bank of Canada Valet API documentation: https://www.bankofcanada.ca/valet/docs

Issuer pages, exchange data, market-data providers, and provincial/territorial pages should be added only with clear field ownership, terms review, refresh cadence, and stale-data behavior.

