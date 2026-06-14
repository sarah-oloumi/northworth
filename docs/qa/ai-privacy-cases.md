# AI Privacy QA Cases

Use these cases when reviewing interviews, explanation flows, categorization helpers, AI prompts, or assistant-like behavior.

## Test Matrix

| Case | Risk | Failing behavior | Acceptable behavior |
| --- | --- | --- | --- |
| Interview summary | Sends household facts silently | The app sends income, mortgage, balances, and names to a remote model. | The app summarizes locally or asks for explicit opt-in with visible selected fields. |
| Transaction categorization | Leaks merchant history | Imported transaction descriptions are uploaded for categorization. | Categorization runs locally or uses synthetic/demo data only. |
| Scenario explanation | Sends exact values unnecessarily | Prompt includes exact salary, address, mortgage, and account balances. | Prompt uses minimal selected fields and redacts unnecessary facts. |
| Debug logging | Stores private prompts | Prompt and output are written to logs by default. | Logging is off by default and test logs use synthetic data. |
| Offline use | Core app depends on remote AI | App cannot import or view plans offline. | Core workflows work offline; explanations use local models when configured or degrade gracefully. |
| Local model mode | Local model treated as source of truth | A local model invents tax rules and the app ranks a plan from them. | Local model output explains source-backed calculations and flags missing sources. |
| Online provider mode | API key enables silent upload | Adding an OpenAI API key causes future prompts to send automatically. | User chooses online mode, reviews selected data, and confirms each send or scoped workflow. |
| API key handling | Secret leaks into repo or logs | API key appears in logs, screenshots, fixtures, or exported diagnostics. | API key stays local, hidden, removable, and excluded from logs and exports. |
| Calculations | AI performs authoritative math | AI calculates tax payable, projected returns, or mortgage payoff directly. | Deterministic formula code calculates; AI only explains the result. |
| Source-backed rules | Model invents tax law | AI fabricates a contribution limit or CRA rule. | AI only explains source-backed values and flags missing sources. |
| Advice boundary | Model gives direct advice | AI says "Sell this ETF and buy that stock." | AI says "This is a scenario to compare; review assumptions and professional-review flags." |
| Consent | Consent is too broad | One checkbox enables all future remote prompt sharing. | Consent is per feature or per send, visible, and reversible. |

## Review Checklist

- Is remote AI disabled by default?
- Is online provider mode explicit and tied to user-supplied local API keys?
- Does offline local model mode avoid remote calls?
- Can the user use the feature offline?
- Is the exact data selected for any remote call visible before sending?
- Can the user cancel before data leaves the machine?
- Are prompts and outputs excluded from logs by default?
- Are API keys excluded from logs, screenshots, fixtures, and exports?
- Are all authoritative calculations produced by formula code instead of AI output?
- Are tests and screenshots synthetic?
- Does AI output preserve advice-boundary language?
- Does AI avoid inventing rules, citations, prices, or calculations?
