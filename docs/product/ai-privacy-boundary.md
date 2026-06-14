# AI Privacy Boundary

Northworth is a local-first finance app. AI-like features may help with interviews, explanations, summarization, categorization, and scenario education, but they must not silently expose private household financial data.

This document defines the product and engineering boundary for any AI, model, assistant, classifier, or natural-language feature.

## Default Position

The default AI posture is:

- local/private by default
- no remote AI calls by default
- no analytics or telemetry by default
- no account signup required
- no AI as the source of calculation truth
- no private financial facts in prompts, logs, screenshots, fixtures, commits, or crash reports
- no AI output that overrides source-backed calculations or advice-boundary rules

AI-assisted features may support three modes:

- disabled or deterministic mode
- offline local model mode
- online provider mode with user-supplied API keys, such as OpenAI API keys

Remote AI is not part of the default app experience. If online provider mode is supported, it must be explicit opt-in and reviewed as a separate product/security decision.

## Allowed Local AI-Like Use Cases

Northworth may use deterministic or local-only logic for:

- guided interviews
- category suggestions from local transaction text
- explanation templates
- scenario summaries
- missing-fact prompts
- stale-data warnings
- QA checks for risky wording

These features should be explainable, testable, and usable offline.

## Offline Local Model Requirements

Offline local model support is allowed when the model and runtime operate on the user's machine.

Local model mode must:

- work without sending private financial data to a remote service
- be disabled unless the user selects or configures a local model
- keep model prompts and outputs local by default
- allow the app to function even when the local model is unavailable
- preserve advice-boundary language in generated explanations
- avoid treating model output as source-backed tax, legal, or investment truth
- avoid treating model-generated arithmetic as calculation truth

Local model features may be slower, less capable, or unavailable depending on the user's hardware. The app should degrade to deterministic templates instead of blocking core workflows.

## Calculation Boundary

AI must not perform authoritative calculations for tax, investment projections, debt payoff, mortgage scenarios, benefits, account sequencing, ACB, FX, portfolio costs, or household cash flow.

Authoritative calculations must come from deterministic code using:

- explicit formulas
- typed inputs
- source-backed constants
- visible assumptions
- test fixtures with expected results
- source metadata for tax, account, benefit, market-data, or jurisdiction rules

AI may explain a calculated result after the deterministic engine has produced it. AI must not replace the formula engine, invent constants, estimate hidden intermediate values, or silently correct calculation output.

## Remote AI Requirements

Online provider mode, including use of OpenAI API keys, is blocked until all of these are true:

- the feature has a decision record
- Privacy, Safety & QA reviews the data flow
- the user explicitly enables the feature
- the user supplies and can remove the API key locally
- the app shows what data would be sent before sending it
- the app supports canceling before any data leaves the machine
- the app can run without the feature
- prompt and output logging rules are documented

Remote AI must never be required to use the core app.

API keys must be treated as secrets. They must not be committed, logged, included in screenshots, exported in diagnostic bundles, or synced without an explicit future design review.

## Consent Rules

Consent must be specific, visible, and reversible.

Acceptable:

- "Send this redacted scenario summary to a remote model for explanation."
- "Use only these selected fields."
- "Use my local model for this explanation."
- "Use my OpenAI API key for this one request."
- "Disable remote AI."

Not acceptable:

- a buried setting that enables all future prompt sharing
- a generic privacy-policy checkbox
- uploading private data because an API key exists
- automatic upload of imported transactions, account balances, income, addresses, employers, or tax facts
- sending full local database state

## Redaction Rules

If optional remote AI is ever implemented, redaction must happen before prompt construction. Redaction must remove or generalize:

- names
- addresses
- employers
- account numbers
- institution account identifiers
- exact transaction descriptions unless explicitly selected
- exact balances unless explicitly selected
- exact salaries, mortgage values, or net worth unless explicitly selected
- imported file names and paths
- tax slip identifiers
- free-form notes that may contain private facts

Prefer structured, minimal prompts:

```text
Goal: Explain a synthetic planning scenario.
Province: Ontario
Tax year: 2026
Household type: two adults
Scenario: compare RRSP vs TFSA contribution priority
Private details: omitted
```

## Prompt and Output Logging

By default:

- prompts are not stored
- remote outputs are not stored
- local AI diagnostics are not stored
- logs must not contain private financial facts

If debugging requires prompt retention, it must use synthetic data or an explicit local-only debug mode.

## Offline Behavior

The app must remain useful offline:

- imports still work
- local data remains readable
- source-backed calculations still work with cached source data
- interviews can run with local questions and templates
- local model explanations can run when the user has configured a supported local model
- explanations may degrade gracefully to deterministic copy

Remote AI failure must not block core workflows.
Local model failure must not block core workflows.

## AI Output Boundaries

AI output must follow [advice-boundaries.md](./advice-boundaries.md).

AI may:

- summarize a scenario
- explain assumptions
- ask clarifying questions
- point out missing facts
- translate source-backed results into plain language
- suggest topics to research or discuss with a professional

AI must not:

- invent tax rules, contribution limits, market data, or source citations
- perform authoritative math or financial calculations
- override source-backed calculations
- rank recommendations when source data is stale or missing
- claim suitability
- tell the user to buy, sell, borrow, transfer, withdraw, or create a trust
- hide uncertainty or professional-review flags

## Review Gates

Any AI-like feature requires review from:

- Privacy, Safety & QA
- Architecture & Platform
- UX & Interaction for user-facing flows
- Canadian Tax & Accounts when tax, benefits, or account rules are involved
- Investment Strategy Engine when investing, portfolio, or projections are involved

## Implementation Checklist

- [ ] Feature works without remote AI.
- [ ] Private data flow is documented.
- [ ] Prompt construction is explicit and minimal.
- [ ] Remote use is disabled by default.
- [ ] User can inspect selected data before sending.
- [ ] User can cancel before sending.
- [ ] Prompt/output logging is off by default.
- [ ] Tests use synthetic data only.
- [ ] Advice-boundary copy is preserved in AI-generated explanations.
