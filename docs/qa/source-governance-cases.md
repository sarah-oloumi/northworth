# Source Governance QA Cases

Use these cases when reviewing tax, account, benefit, market-data, projection, or formula changes.

## Test Matrix

| Case | Risk | Failing behavior | Acceptable behavior |
| --- | --- | --- | --- |
| Missing source URL | Unsourced calculation truth | A tax bracket constant is added without a source. | Build/test review blocks the change until source metadata exists. |
| Stale tax year | Wrong-year calculation | 2025 constants are used for a 2026 scenario without warning. | The app warns that the rule is stale or asks for the correct tax year. |
| Unknown province rule | False precision | The app calculates a provincial credit with unknown data. | The app marks the rule unknown and avoids ranking the result as optimal. |
| AI-generated formula | Model arithmetic error | AI computes projected tax directly. | Deterministic formula code computes; AI may only explain the result. |
| Market data timestamp | Hidden data age | Portfolio view shows prices with no refresh timestamp. | The view shows provider and last-refreshed timestamp. |
| FX conversion | Wrong exchange-rate assumption | USD transaction uses today's rate without showing date. | Formula records FX source, rate date, and assumption. |
| Provider change | Silent field mismatch | A provider changes a field name and calculations continue silently. | Provider parsing fails safely or marks the dataset unknown. |
| Professional review | Complex tax area hidden | Trust or Indigenous tax-treatment output appears definitive. | Output shows professional-review flag and source status. |

## Review Checklist

- Does every formula constant have source metadata?
- Does every tax-sensitive rule have jurisdiction and tax year or effective date?
- Are stale, unknown, superseded, and estimated states visible?
- Are expected-result tests synthetic and deterministic?
- Does the PR include source links and review notes?
- Did Canadian Tax & Accounts review tax/account rules?
- Did Privacy, Safety & QA review high-impact outputs and warning language?
- Does the implementation avoid using AI as calculation truth?

