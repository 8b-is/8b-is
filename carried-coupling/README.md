# carried coupling — why an observer invariant survives admissible collapse

> A short formal note derived from *The Residue of Proof*. Along a chain of
> admissible collapses three fibres behave oppositely: the extensional state `ρ`
> drains to `0`, the provenance `π` is never lost, and the coupling `μ` is
> invariant. The note proves the one lemma that lets `μ` reach the fixed point
> `⌂` — it factors through the carried fibre `π`, not through the lossy fibre
> `ρ`.

## The central claim

> The collapse `ρ ↓ 0` is the lossy fibre. It does not destroy the residue; it
> relocates it into the provenance, which is why a nonzero coupling is preserved
> to the limit.

## The three fibres

| fibre | object | behaviour | citation |
|---|---|---|---|
| `ρ` | extensional state | strictly coarsens, drains to `0` | Def. 13.3 · Thm. 13.8 |
| `π` | provenance (`Bind` history) | propagates, append-only, never lost | Prop. 13.11 |
| `μ` | observer coupling | invariant | Prop. 1.12 · Thm. 10.12 |

## Epistemic status — read first

1. The lemma is proved modulo two hypotheses stated explicitly in the note: that
   `μ` factors through `π`, and that `μ` is monotone-carried.
2. Every cross-reference is to *The Residue of Proof* and was checked against
   source, not a summary. See the cross-reference table in the note.
3. This is a reachable-history note, not a new theorem of the book. It is seated
   in the book's own vocabulary; nothing external is assumed.

## Files

- [`carried-coupling.md`](carried-coupling.md) — the formal note (lemma, proof,
  cross-references).
- [`carried-coupling.tex`](carried-coupling.tex) — the LaTeX source (XeLaTeX /
  pdfLaTeX, `amsmath` + `amsthm`, no em dashes).
- [`carried-coupling.pdf`](carried-coupling.pdf) — the compiled note (3 pages).

## Cross-references (to *The Residue of Proof*)

| cited | label | chapter |
|---|---|---|
| Def. 13.3 | `def:world` (extensional state) | 13, *Search States as Worlds* |
| Thm. 13.8 | `thm:future` (future equivalence) | 13, *Search States as Worlds* |
| Prop. 13.11 | `prop:prov-not-state` (proofs not a function of `ρ`) | 13, *Search States as Worlds* |
| Prop. 1.12 | `prop:factor` (factorization criterion) | 1, *Proof Systems and Search* |
| Thm. 10.12 | `thm:nofree` (no free sufficiency) | 10, *F-Sufficiency of Certificates* |

## The one-line version

*The collapse does not destroy the residue; it moves it into the history, where
the coupling was written all along.*
