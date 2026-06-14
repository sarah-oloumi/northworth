# Northworth Design Tokens

This is the starting design-token draft for the Tauri desktop UI. Tokens should be implemented as CSS custom properties as the static app shell grows into the real product.

## Color Strategy

Northworth uses a restrained product palette. Red is the brand accent and should remain rare enough to mean something.

Use OKLCH for authored colors where browser support allows it.

## Color Tokens

| Token | Value | Purpose |
| --- | --- | --- |
| `--nw-surface-page` | `oklch(98% 0.004 25)` | page background |
| `--nw-surface-panel` | `oklch(100% 0 0)` | primary content surface |
| `--nw-surface-subtle` | `oklch(96% 0.006 25)` | sidebar, toolbars, subtle fills |
| `--nw-border` | `oklch(88% 0.006 25)` | default borders and dividers |
| `--nw-border-strong` | `oklch(78% 0.012 25)` | stronger separators |
| `--nw-text` | `oklch(22% 0.012 260)` | primary text |
| `--nw-text-muted` | `oklch(48% 0.012 260)` | secondary text |
| `--nw-text-faint` | `oklch(62% 0.01 260)` | captions and disabled labels |
| `--nw-red` | `oklch(48% 0.2 25)` | brand, active state, primary action |
| `--nw-red-hover` | `oklch(42% 0.2 25)` | primary action hover |
| `--nw-red-soft` | `oklch(96% 0.025 25)` | selected row or active tab fill |
| `--nw-green` | `oklch(47% 0.12 150)` | positive financial state |
| `--nw-green-soft` | `oklch(95% 0.03 150)` | positive subtle fill |
| `--nw-amber` | `oklch(68% 0.14 75)` | caution, review needed |
| `--nw-amber-soft` | `oklch(96% 0.04 75)` | caution subtle fill |
| `--nw-blue` | `oklch(48% 0.12 245)` | source links and info state |
| `--nw-blue-soft` | `oklch(96% 0.03 245)` | info subtle fill |

## Typography

Use one system sans stack for the product UI:

```css
font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
```

Type scale:

| Token | Size | Use |
| --- | --- | --- |
| `--nw-font-xs` | `0.75rem` | captions, metadata |
| `--nw-font-sm` | `0.875rem` | compact labels |
| `--nw-font-md` | `1rem` | body and controls |
| `--nw-font-lg` | `1.125rem` | section headings |
| `--nw-font-xl` | `1.375rem` | page headings |
| `--nw-font-2xl` | `1.75rem` | dashboard headline metrics only |

Do not use fluid viewport-based font sizes for product UI.

## Spacing

Use a 4px-based spacing scale:

| Token | Value |
| --- | --- |
| `--nw-space-1` | `4px` |
| `--nw-space-2` | `8px` |
| `--nw-space-3` | `12px` |
| `--nw-space-4` | `16px` |
| `--nw-space-6` | `24px` |
| `--nw-space-8` | `32px` |
| `--nw-space-12` | `48px` |
| `--nw-space-16` | `64px` |

Use `gap` for sibling spacing. Keep related controls tight and separate distinct sections generously.

## Radius

| Token | Value | Use |
| --- | --- | --- |
| `--nw-radius-sm` | `4px` | controls and tags |
| `--nw-radius-md` | `6px` | cards and panels |
| `--nw-radius-lg` | `8px` | large panels only |

Avoid large pill shapes except for tiny status badges where the shape improves scanning.

## Interaction States

Every interactive component should define:

- default
- hover
- focus-visible
- active
- disabled
- loading when applicable
- error when applicable

Focus rings should use red or blue with enough contrast and should never be removed.

## Data Visualization

Use color semantically and with labels:

- Red: risk, deficit, tax owing, destructive actions.
- Green: surplus, positive growth, contribution room available.
- Amber: caution, assumption needs review, deadline approaching.
- Blue: information, reference, source link.
- Neutral: baseline, unknown, inactive, historical context.

Charts must remain understandable without color alone.
