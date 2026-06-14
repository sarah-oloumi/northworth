# Advice, Tax, and Recommendation Boundaries

Northworth is a local-first planning and education app. It can help users understand scenarios, compare assumptions, and prepare better questions. It must not present itself as a registered financial advisor, tax preparer, lawyer, accountant, broker, portfolio manager, or trading system.

This document defines product-language boundaries for recommendation-like features. It is a product and safety standard, not legal advice.

## Product Position

Northworth may:

- organize user-provided financial facts
- import and categorize local financial data
- show calculations from source-backed rules
- compare scenarios using visible assumptions
- explain tradeoffs, uncertainty, and missing facts
- suggest areas to research or discuss with a qualified professional
- flag stale data, incomplete inputs, and professional-review situations

Northworth must not:

- execute trades or connect to brokerage order-entry systems
- claim that a plan is suitable for a specific person in the regulated sense
- claim to provide legal, tax, accounting, insurance, mortgage, or securities advice
- claim certainty about market returns, tax outcomes, benefits eligibility, or CRA/provincial treatment
- hide assumptions, source dates, stale data, or confidence limits
- encourage false reporting, tax evasion, artificial facts, or unsupported aggressive tax positions

## Required In-App Boundary Language

Use short, plain language near high-impact outputs:

> Northworth is an educational planning tool. It shows scenarios based on your inputs, assumptions, and source-backed rules. It does not provide legal, tax, accounting, insurance, mortgage, or investment advice.

Use stronger language when an output may affect tax filings, trades, leverage, trusts, insurance, real estate, or registered-account withdrawals:

> This scenario may have tax, legal, or investment consequences. Review the assumptions and sources, then consider a qualified professional before acting.

Use stale-data language when a source is old, missing, or unverified:

> Some rules or market data used here may be stale or incomplete. Treat this as a planning prompt, not a decision.

## Allowed Product Language

Prefer language that frames outputs as scenarios, comparisons, and research prompts.

- "This scenario estimates..."
- "Based on the assumptions shown..."
- "A lower-tax path to compare is..."
- "This may be worth discussing with a CPA, lawyer, or registered advisor."
- "This option appears more tax-efficient under these assumptions."
- "This result depends on your province, account ownership, tax year, and source dates."
- "Northworth cannot confirm eligibility from the facts provided."
- "Professional review is recommended before acting."

## Banned Product Language

Avoid language that implies regulated advice, certainty, or guaranteed suitability.

- "You should buy..."
- "You should sell..."
- "This is the best investment for you."
- "This will minimize your tax."
- "This guarantees..."
- "CRA will accept..."
- "This trust strategy is right for you."
- "Withdraw from your RRSP now."
- "Move this money to your partner's account."
- "This plan is suitable for your household."
- "No professional review is needed."

## High-Impact Output Requirements

A high-impact output is any output that could reasonably influence a user to change taxes, investments, debt, insurance, estate plans, real estate, or registered-account behavior.

Every high-impact output must show:

- scenario name and purpose
- user inputs used
- assumptions used
- source links and last-reviewed dates for tax/rule data
- market-data timestamp where relevant
- missing facts and uncertainty
- stale-data warnings where relevant
- professional-review flag where relevant
- explanation of tradeoffs, not just a ranked answer

High-impact outputs must be reviewable by Privacy, Safety & QA before release.

## Human Review Triggers

The app should require prominent professional-review language for:

- trusts, estate planning, or beneficiary strategy
- Indigenous tax-treatment scenarios
- foreign property, T1135 awareness, or cross-border tax facts
- RRSP/RRIF/LIRA/LIF withdrawals
- non-registered capital gains, superficial losses, or ACB uncertainty
- rental property, change-in-use, or principal residence exemption scenarios
- deductible interest, HELOC, or leveraged investing scenarios
- incorporated, contractor, or self-employed planning
- spousal/common-law transfers, income splitting, or attribution-rule scenarios
- disability, dependency, RESP, RDSP, or benefit/clawback scenarios

## Source Rules

Tax, account, benefit, and market-data logic must not rely on unsourced model memory. Source-backed rule data must include:

- jurisdiction
- tax year or effective date
- source URL
- last reviewed date
- status: `current`, `stale`, `unknown`, or `superseded`

If a rule is `stale` or `unknown`, Northworth may still show an educational note, but it must not rank or optimize using that rule without a warning.

## Privacy Rules

Private user facts must stay local by default. Private household data must not be sent to market-data providers, analytics systems, AI services, or remote APIs unless the user explicitly configures and consents to that behavior.

Remote AI or cloud features are out of scope until the AI privacy boundary is defined.

## Official References

Use official or regulator sources first when developing finance/tax boundaries:

- Canadian Securities Administrators, National Registration Search: https://www.securities-administrators.ca/investor-tools/check-registration/
- Ontario Securities Commission, Check before you invest: https://www.osc.ca/en/investors/check-before-you-invest
- Canadian Investment Regulatory Organization, Check an advisor: https://www.ciro.ca/investors/how-make-complaint/investor-protection/check-advisor
- Financial Services Regulatory Authority of Ontario, Financial planners and financial advisors title protection: https://www.fsrao.ca/industry/financial-planners-and-financial-advisors
- Canada Revenue Agency, Individuals and income tax: https://www.canada.ca/en/revenue-agency/services/tax/individuals.html
- Canada Revenue Agency, Forms and publications: https://www.canada.ca/en/revenue-agency/services/forms-publications.html

