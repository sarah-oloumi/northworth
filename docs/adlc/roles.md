# ADLC Team Charter

Northworth uses an Agentic Development Lifecycle (ADLC) operating model for planning, reviewing, and improving work. The "agents" are specialist team perspectives, not fake people. A contributor can play more than one role, but issues and pull requests should make the active team perspective visible.

Every team has a purpose, owned work, boundaries, review partners, and a lesson-learned requirement. This keeps financial, tax, privacy, and architecture decisions explicit instead of hidden in chat history.

## Operating Rules

- Hire: every team has a role, scope, boundary, review partner, and lesson-learned requirement.
- Onboard: each team uses only the source material and repo context needed for its task.
- Coach: new workflows get fixtures, evaluation cases, and examples before they are trusted.
- Supervise: high-impact finance, tax, privacy, and investment outputs require source links and review.
- Team: specialists hand off through issues and pull requests, not hidden context.
- Improve: every epic closes with a short lessons-learned note.

## Teams

### Product & ADLC Ops

- Label: `team: product-ops`
- Purpose: keep work small, sequenced, reviewable, and aligned to local-first privacy.
- Owns: backlog hygiene, milestone scope, dependencies, acceptance criteria, release readiness.
- Review partners: Privacy, Safety & QA; Architecture & Platform.
- Boundaries: does not approve financial, tax, or legal logic alone.
- Lesson learned: record what slowed delivery, what was unclear, and what should change next cycle.

### Architecture & Platform

- Label: `team: architecture`
- Purpose: preserve simple, testable architecture across Rust/Tauri, domain modules, persistence, imports, and UI boundaries.
- Owns: package boundaries, data flow, storage strategy, security posture, dependency tradeoffs.
- Review partners: Senior Developer; Privacy, Safety & QA.
- Boundaries: does not introduce heavy dependencies or remote services without explicit justification.
- Lesson learned: record architectural assumptions that held, broke, or need proof.

### Data Ingestion

- Label: `team: data-ingestion`
- Purpose: turn bank and brokerage exports into trusted local data.
- Owns: CSV, OFX/QFX, brokerage imports, transaction normalization, import review, synthetic fixtures.
- Review partners: Architecture & Platform; Privacy, Safety & QA.
- Boundaries: does not store credentials or send imported financial data to third parties.
- Lesson learned: record format quirks, parsing risks, fixture gaps, and user-review improvements.

### Canadian Tax & Accounts

- Label: `team: canadian-tax`
- Purpose: define source-backed Canadian tax, account, benefit, jurisdiction, and reporting rules.
- Owns: CRA/provincial/territorial source catalog, registered/unregistered account rules, tax calculations, benefits, foreign-property awareness, trusts, Indigenous tax-treatment requirements.
- Review partners: Privacy, Safety & QA; Investment Strategy Engine.
- Boundaries: does not present app output as personalized regulated tax or financial advice.
- Lesson learned: record source gaps, ambiguous rules, stale data risks, and professional-review boundaries.

### Investment Strategy Engine

- Label: `team: investment-engine`
- Purpose: build explainable scenario engines for portfolio costs, risk, projections, sequencing, asset location, rebalancing, and tax-efficient planning.
- Owns: risk tolerance, risk capacity, ETF/stock/hybrid comparison, contribution/withdrawal sequencing, projections, ACB/tax lots, CAD/USD tradeoffs.
- Review partners: Canadian Tax & Accounts; Privacy, Safety & QA.
- Boundaries: does not execute trades or claim certainty about future returns.
- Lesson learned: record model assumptions, sensitivity risks, edge cases, and misleading-output risks.

### Household Planning

- Label: `team: household-planning`
- Purpose: model real household complexity across one or two adults, dependents, property, debt, income shocks, equity compensation, estate/insurance needs, and emergency funds.
- Owns: household profile, person-level ownership, dependents, mortgage/debt, real estate, equity compensation, survivorship and insurance scenarios.
- Review partners: Canadian Tax & Accounts; Investment Strategy Engine.
- Boundaries: does not merge assets, income, or tax identity without explicit user-provided facts.
- Lesson learned: record missing household facts, scenario pitfalls, and assumptions that materially changed outcomes.

### Privacy, Safety & QA

- Label: `team: privacy-safety`
- Purpose: verify correctness, privacy, edge cases, explainability, and user trust.
- Owns: test strategy, synthetic fixtures, source review gates, advice-boundary review, encryption-at-rest decisioning, AI privacy boundary, audit trails.
- Review partners: every team.
- Boundaries: blocks release when outputs are unsupported, stale, misleading, or privacy-risky.
- Lesson learned: record failure modes, missed cases, new tests, and safety language improvements.

### UX & Interaction

- Label: `team: ux`
- Purpose: make complex financial planning understandable, calm, and navigable.
- Owns: information architecture, import review UX, references tab, interview flows, dashboards, stale-data indicators, scenario explanations.
- Review partners: Product & ADLC Ops; Privacy, Safety & QA.
- Boundaries: does not hide assumptions, uncertainty, source status, or professional-review flags for visual simplicity.
- Lesson learned: record confusing flows, copy risks, and places where users need better guidance.

## Review Gates

Use these gates when a pull request touches the matching area.

- Finance/tax rules: Canadian Tax & Accounts plus Privacy, Safety & QA.
- Investment scenarios: Investment Strategy Engine plus Canadian Tax & Accounts.
- Private local data: Privacy, Safety & QA plus Architecture & Platform.
- Imports/parsers: Data Ingestion plus Privacy, Safety & QA.
- User-facing planning flows: UX & Interaction plus the owning domain team.
- Architecture decisions: Architecture & Platform plus Product & ADLC Ops.

## Lessons Learned Format

Use this format in epic closeout comments and meaningful pull requests:

```markdown
## Lessons Learned

- What changed in our understanding:
- What slowed us down:
- What risk or edge case did we discover:
- What should the next issue or PR do differently:
```

Small documentation or mechanical changes may write `N/A - mechanical change` when there is no meaningful lesson.

