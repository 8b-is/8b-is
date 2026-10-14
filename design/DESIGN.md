# 8b.is — design system

The canonical visual language for the 8b-is constellation. One palette, one
type system, one motion rule. Everything else is a surface that wears it.

> **Ink over chrome.** The constellation's identity is a near-black canvas and
> a small set of luminous accents. We do not decorate; we annotate. A result
> should look like a measuring instrument, not a billboard.

---

## 1. Colour

### Core palette (the "constellation ink")

| token | hex | role |
|-------|-----|------|
| `--bg` | `#05060a` | the canvas — near-black, never pure black |
| `--panel` | `#0b0f19` | raised surface (cards, nav) |
| `--panel-2` | `#0e1424` | second elevation (nested cards) |
| `--ink` | `#cfc9dd` | primary text |
| `--dim` | `#7f7c99` | secondary text, labels |
| `--faint` | `#5b577a` | tertiary text, captions |
| `--line` | `rgba(170,150,255,0.18)` | hairline borders |

### Accents

| token | hex | role |
|-------|-----|------|
| `--mint` | `#62e6c9` | **primary** accent — links, CTAs, "ours" |
| `--gold` | `#e8b05c` | secondary accent — the mark, the sun, sponsor |
| `--violet` | `#b48bff` | tertiary accent — constellation, memory |
| `--cyan` | `#00f0ff` | signal accent — data, output, "readout" |
| `--pink` | `#ff4d9d` | warning / emphasis only, used sparingly |

### Rules

- Background is always `--bg` with an optional single radial vignette to `#0d1220`.
- Accents are never used as large fills. They are strokes, glyphs, 1px borders,
  and small text. The one exception is a solid `--mint` primary button.
- Never more than two accents in a single card.
- Text on `--bg` must clear WCAG AA: `--ink` on `--bg` = 12.4:1, `--dim` = 4.6:1.

---

## 2. Type

| role | family | notes |
|------|--------|-------|
| display / UI | `ui-monospace, "SF Mono", Menlo, monospace` | headings, labels, buttons, numbers |
| prose | `Georgia, serif` | body copy, the human voice |
| CJK | `"STKaiti", "KaiTi", "PingFang SC", serif` | zeno, Japanese/mandarin surfaces |

- Kickers: 11px, `letter-spacing: 0.32em`, uppercase, `--dim`.
- Headings: monospace, `letter-spacing: -0.01em`, weight 600.
- Body: 16–18px, `line-height: 1.7`.
- Numbers and code always monospace; a metric is never serif.

---

## 3. Space & shape

- Radius: `10px` small, `14px` cards, `999px` pills.
- Borders: `1px solid var(--line)`; accent borders are the accent at ~35–50% alpha.
- Section rhythm: `84px` vertical padding desktop, `64px` mobile. Max width `1376px`.
- Grid: `repeat(auto-fill, minmax(280px, 1fr))` for card shelves.
- Every interactive element has a visible `:focus-visible` outline in `--mint`.

---

## 4. Motion

- **Live-drawn, never baked.** Surfaces may draw themselves (canvas wave fields,
  procedural glyphs) but never ship a video.
- Durations: `150ms` for hover/state, `18–26s` for ambient loops.
- Respect `prefers-reduced-motion: reduce` — always render a static frame
  instead, and cancel animation loops.
- Pause animation when `document.hidden`.

---

## 5. Components

| component | spec |
|-----------|------|
| `card` | `--panel` at 2% white, `--line` border, radius 14, hover → accent border + `translateY(-2px)` |
| `pill` | radius 999, `--line` border, `--dim` text, active → solid `--mint` on `--bg` text |
| `kicker` | 11px monospace, uppercase, `0.32em` tracking, `--dim` |
| `primary-button` | solid `--mint`, `--bg` text, radius 10, hover lift + mint glow |
| `ghost-link` | `--mint` text, 1px mint underline at 35% |
| `sigil` | a single monospace glyph per project, in the project's accent |

---

## 6. The mark

`8b.` — monospace, tight negative tracking (`-0.08em` to `-0.13em`), with the
full stop in `--gold`. Never stretch, never outline, never place on a light
field without a dark plate.

---

## 7. Voice

- Lowercase kickers, sentence-case prose, no exclamation marks in body.
- Numbers always carry their hedge. If a figure is a single run, say so.
- "Measure, don't assume" — state the metric, not the caption.

---

*8b-is · the constellation ink · vaked.dev · fine touch from within*
