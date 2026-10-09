# Carried Coupling

**Why an Observer Invariant Survives Admissible Collapse**

*Flyxion · Independent Researcher · October 2026*

A reachable-history note on *The Residue of Proof: Sufficiency, History, and
Repair in Automated Reasoning*. Source: [`carried-coupling.tex`](carried-coupling.tex)
· PDF: [`carried-coupling.pdf`](carried-coupling.pdf).

## Abstract

A chain of admissible collapses is written

```
∅R ⟶ᴸ R₁ ⟶ᴸ R₂ ⟶ᴸ ⋯ ⟶ᴸ Rₙ ⟶ᴸ ⌂
```

Along it three fibres of a run behave oppositely: the extensional state `ρ`
strictly coarsens and drains to `0`; the provenance `π` propagates and is never
lost; and the coupling `μ` between a result and an observer is invariant. This
note isolates the mechanism that lets `μ` reach the terminal object `⌂`: `μ`
factors through `π` and not through `ρ`, and `π` is carried across each arrow.
The result is a short lemma, the *carried coupling*, seated against the
cross-references of *The Residue of Proof*. The point is not that collapse is
harmless but that it relocates the residue from the lost fibre into the carried
one, which is why a nonzero coupling is preserved to the limit.

## 1. Setting

Fix a chain of representations in the sense of *The Residue of Proof*, with a
declared task family at each stage. The chain

```
∅R ⟶ᴸ R₁ ⟶ᴸ R₂ ⟶ᴸ ⋯ ⟶ᴸ Rₙ ⟶ᴸ ⌂
```

is a sequence of admissible collapses. Each arrow `⟶ᴸ` is a representation map
that is sufficient for its declared family and refusal-respecting; the terminal
`⌂` is the fixed point at which no further collapse is demanded.

Three quantities travel along the chain, and three results of the book fix their
behaviour.

- The **extensional state** `ρ` is the triple `(D, R, P)` of derived, refused,
  and popped items (Definition 13.3, *Events, histories, state*). It strictly
  coarsens and drains to `0`: it is the lossy fibre.
- The **provenance** `π` is the set of `Bind` events recorded in the history. It
  is append-only, and it is not a function of the extensional state
  (Proposition 13.11, *Proofs are not a function of the extensional state*).
- The **coupling** `μ(R, you)` is the quantity of interest between a
  representation `R` and the observer. It is invariant on the chain.

| fibre | object | behaviour | citation |
|---|---|---|---|
| `ρ` | extensional state | strictly coarsens, drains to `0` | Def. 13.3 · Thm. 13.8 |
| `π` | provenance (`Bind` history) | propagates, append-only, never lost | Prop. 13.11 |
| `μ` | observer coupling | invariant | Prop. 1.12 · Thm. 10.12 |

## 2. Why `μ` factors through `π` and not through `ρ`

Chapter 1 fixes when a task can be read off a representation: a task factors
through a map exactly when it is constant on that map's fibres.

> **Proposition (Factorization criterion; Prop. 1.12).** Let `ϱ : X → D` be a
> representation and `T : X → Y` a task. There is `T' : D → Y` with
> `T = T' ∘ ϱ` if and only if `T` is constant on every fibre of `ϱ`.

The extensional state `ρ` is sufficient only for tasks constant on its fibres.
The theorem of Chapter 10 makes the converse unconditional: a non-injective
representation is never sufficient for the family of all `{0,1}`-valued tasks.

> **Theorem (No free sufficiency; Thm. 10.12).** Let `σ : H → R`. Then `σ` is
> injective if and only if it is sufficient for the family of all functions
> `H → {0,1}`. In particular, if `σ` identifies two distinct histories then some
> `{0,1}`-valued task is not served.

The coupling `μ` distinguishes runs that share an output, so it does not factor
through `ρ`. This is exactly the failure recorded for the extensional state in
Chapter 13.

> **Proposition (Proofs are not a function of the extensional state; Prop. 13.11).**
> There are well-formed histories `H₁, H₂` with `ρ(H₁) = ρ(H₂)` and
> `π(H₁) ≠ π(H₂)`, in which the recorded justifications of an item yield
> derivations with different leaves. Hence the task that returns a derivation is
> not `ρ`-sufficient, while the extensional state is minimal for all future
> behaviour of the search (Thm. 13.8, *Future equivalence*).

So `μ` is not constant on the fibres of `ρ`, and by the factorization criterion it
admits no reading from `ρ`. The proof lives in `π`, not in the extensional state;
the coupling rides the same carried fibre.

## 3. The carried coupling

Let the chain of collapses act on representations as
`ρ₀ ⊇ ρ₁ ⊇ ⋯ ⊇ ρₙ ↓ 0` for the extensional state, and as
`π₀ ↝ π₁ ↝ ⋯ ↝ πₙ ↝ π⌂` for the provenance, where each `↝` is an append:
`πᵢ ⊆ πᵢ₊₁`.

Say `μ` **factors through `π`** if there is `μ̄` with
`μ(R, you) = μ̄(π(R), you)`, and that `μ` is **monotone-carried** if `μ̄` does
not vanish under the appends that extend `π` to `π⌂`: whenever
`μ̄(πᵢ, you) ≠ 0` and `πᵢ ⊆ πᵢ₊₁` then `μ̄(πᵢ₊₁, you) ≠ 0`.

> **Lemma (Carried coupling).** Suppose `μ` factors through `π`, is
> monotone-carried, and `π` survives each arrow `⟶ᴸ` (that is, `πᵢ ↝ πᵢ₊₁` at
> every stage and `πₙ = π⌂`). If `μ(Rᵢ, you) ≠ 0` for every `i` along the
> chain, then `μ(⌂, you) ≠ 0`.

*Proof.* Because `μ = μ̄ ∘ π`, we may read the hypothesis as
`μ̄(πᵢ, you) ≠ 0` for all `i`. Induct along the chain. For the base,
`μ̄(π₀, you) ≠ 0`. For the step, assume `μ̄(πᵢ, you) ≠ 0`. The arrow `⟶ᴸ`
collapses only the extensional state: by Proposition 13.11 the provenance is
untouched, so the step is the append `πᵢ ⊆ πᵢ₊₁`. Monotone-carry gives
`μ̄(πᵢ₊₁, you) ≠ 0`. At the last stage `πₙ = π⌂`, so
`μ(⌂, you) = μ̄(π⌂, you) ≠ 0`. ∎

*Remark.* The proof is the whole content: `μ` is constant on the fibres of `π`
rather than those of `ρ`; `π` is the sufficient fibre; and `π` is carried, so the
coupling rides it to the limit. Nothing is claimed about `ρ` except that it is
the wrong fibre to read the coupling from.

## 4. The residue relocates

The collapse `ρ ↓ 0` is the lossy fibre. It does not destroy the residue; it
relocates it into the provenance, which is why the coupling reaches `⌂` intact.
This is the chain-level counterpart of the residue of Chapter 1: the proof
extracted from a run is a function of `π`, the extensional state is a function
of the run, and neither determines the other. A prover that collapses its state
to `ρ` after every iteration, for memory, can continue indefinitely and can still
report that `⊥` was derived, but cannot say how. What survives is exactly the
carried fibre, and the observer coupling is written there.

The formal point, stated once: admissibility of a collapse is relative to a
declared task and a declared continuation; the extensional state is minimal for
all futures (Thm. 13.8); the provenance is the sufficient fibre for proof
extraction (Prop. 13.11); and an observer invariant that factors through the
carried fibre is preserved by composition to the fixed point.

## Status and scope

The lemma is proved modulo two hypotheses stated explicitly: that `μ` factors
through `π`, and that `μ` is monotone-carried. The first is the content of
Section 2, where the factorization criterion (Prop. 1.12) and the failure of the
extensional state (Prop. 13.11) together force `μ` off `ρ`; the second makes
precise the phrase "`π` propagates, append-only, never lost." The reduction
`ρ ↓ 0` is the lossy fibre and the residue relocates into `π`; everything else is
machinery already seated in *The Residue of Proof*. The lemma is offered as a
reachable-history note, not as a new theorem of the book.

## Cross-references (to *The Residue of Proof*)

| cited | label | location |
|---|---|---|
| Def. 13.3 | `def:world` (extensional state) | Chapter 13, *Search States as Worlds*, `13-search-worlds.tex:26` |
| Thm. 13.8 | `thm:future` (future equivalence) | Chapter 13, `13-search-worlds.tex:88` |
| Prop. 13.11 | `prop:prov-not-state` (proofs not a function of `ρ`) | Chapter 13, `13-search-worlds.tex:123` |
| Prop. 1.12 | `prop:factor` (factorization criterion) | Chapter 1, *Proof Systems and Search*, `01-proof-systems.tex:138` |
| Thm. 10.12 | `thm:nofree` (no free sufficiency) | Chapter 10, *F-Sufficiency of Certificates*, `10-sufficiency.tex:127` |

## The one-line version

*The collapse does not destroy the residue; it moves it into the history, where
the coupling was written all along.*
