# Project Instructions

## Purpose

Build a Canadian household wealth-building planner inspired by Portage's transparency and local-first posture, but focused on accumulation, tax efficiency, monthly cash flow, and investment decision support.

## Initial Scope

- Ontario tax assumptions first.
- Two-person household support first.
- Monthly cash flow and annual tax projections.
- Registered and non-registered account strategy comparison.
- Employer compensation modeling, including salary, bonus, RSUs, DPSP, group RRSP, and pension adjustments where applicable.
- Property and mortgage modeling.
- Public market data through a provider abstraction.

## Out Of Scope Until Explicitly Designed

- Tax filing.
- Financial advice claims.
- Account aggregation through banking credentials.
- Publishing or syncing private user data.
- Multi-user cloud accounts.

## Tax Guidance Boundary

The app should help users understand legal tax efficiency. It must not suggest tax evasion, hiding income, falsifying deductions, or misrepresenting residency, ownership, expenses, or account eligibility.

## Data Rules

- Use fictional fixtures only.
- Keep private household files under `private/`, which is ignored by git.
- Keep provider API keys in local environment files only.
- Any sample values in source must be invented and clearly generic.

## Engineering Direction

- Prefer deterministic calculation engines with tests.
- Keep tax constants source-linked and year-labeled.
- Separate domain calculations from UI.
- Separate public market data lookups from private household state.
- Make all assumptions visible to the user.

