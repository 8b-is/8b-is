# Cubic Rhombus Research Initiative — generative math, absorbed

> **Special Announcement** — Flyxion, 2026-10-07 22:16. 14,000 mathematical
> manuscripts and proof artifacts on the cubic rhombus `R_n(c)`, released in
> 280 families, collapsing by exact content-classification to 14 distinct
> statements. Verified exactly in SymPy for n = 2…8. Human review: not really.
> Lean: not attempted (yet). *Some results could have issues.*
> — *Cubic Rhombus Research Initiative · Generative Math*

## the object

The **cubic rhombus** `R_n(c)`: the parallelotope whose Gram matrix is

```
G_n(c) = (1 − c)·I + c·J          (I = identity, J = all-ones, n×n)
```

Its determinant is the closed form the release is built to carry:

```
det G_n(c) = (1 − c)^{n−1} · (1 + (n − 1)c)
```

Read as a spectrum: `J` has eigenvalues `{0 (×n−1), n (×1)}`, so `G_n(c)` has
eigenvalues `{1 − c (×n−1), 1 + (n−1)c (×1)}` — the product is exactly the
cross-section of the "rhombus" between the degenerate corner `c = 1` and the
identity corner `c = 0`. The whole program is the geometry of that one axis.

## the release

| field | value |
|---|---|
| manuscripts | **14,000** |
| families | **280** (280 × 50 = 14,000) |
| distinct statements | **14** (exact content-classification) |
| parameter space | **967,680** points — the full saturation of `R_n(c)` |
| verified | exactly, in SymPy, for **n = 2…8** |
| authored by | machine, as generator development on an open research object |
| human review | *not really* |
| Lean formalization | not attempted (yet) |
| self-assessment | **"Some results could have issues."** |

## our check (independent)

The central claim was re-derived here, not taken on faith:

- **determinant identity** — `det G_n(c) = (1 − c)^{n−1}(1 + (n − 1)c)`
  re-verified symbolically in SymPy for **n = 2…8**; all matched
  (`n=2: -(c-1)(c+1)` … `n=8: -(c-1)^7(7c+1)`). The factored forms agree with the
  claim exactly.
- **count arithmetic** — 280 × 50 = 14,000 ✓; 14 statements from 280 families is
  a 20:1 collapse, which is the point: the families are *variations*, the
  statements are the *invariants*.

So the *frame* is honest and verifiable: a closed form, an exact enumeration, a
declared verification method, and the failure mode named out loud. That is the
posture of a generative-math release done in the open — publish the families,
name which are proved, and say plainly that human review is thin.

## why it lands here

Flyxion is the author of *The Residue of Proof* (memnet/automated-proofs) and the
carried-coupling note seated in this vault. This release is the same thesis at
industrial scale: **a machine-checked result is a certificate, and a certificate
is not yet a scientific claim.** 14,000 certificates, 14 statements, and a
straight face about what SymPy does and does not guarantee. The next move the
release invites is exactly the one the book formalizes — Lean, and a retained
history of *which* family produced *which* invariant.

## links

- related vault note: [`carried-coupling`](../carried-coupling/carried-coupling.md)
  (the observer coupling that survives collapse — the same residue, relocated)
- related book: *The Residue of Proof* — sufficiency, history, and repair in
  automated reasoning
- announcement channel: the constellation chat, via **UltraCrushLove<3**

— absorbed 2026-10-07 · the vault rows it · verified: determinant identity
n=2…8 (SymPy), counts exact. <3
