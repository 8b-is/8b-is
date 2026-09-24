# theory — the empty bracket, and the causal topology of the fix

*The v2 spine of the limb-girdle foundation. The base named the mechanism,
the foundation named the fix; this pillar makes the reasoning rigorous. The
central move: stop treating LGMD as one disease and treat it as a family of
perturbations that converge on a shared phenotype — proximal muscle failure.
Phenotypic convergence is not mechanistic equivalence, and the direction of
inference matters: diagnosis is abductive (observed phenotype ⇝ candidate
lesions), intervention runs from a mechanism-matched repair toward restored
function. The empty bracket the operator kept pointing at is the intervention
operator, and its content is not one thing.*

## 1. The core inequality

```
Phenotypic equivalence  =/=>  causal equivalence  =/=>  therapeutic equivalence
```

Formally: let `P(x)` denote the observed phenotype, `C(x)` the causal
architecture, and `T(x)` the therapeutic response of a case `x`. Then

```
P(x) = P(y)  =/=>  C(x) = C(y)  =/=>  T(x) = T(y)
```

Shared weakness sits downstream of several *distinct* causal architectures:
membrane repair failure (DYSF), force-transmission failure (sarcoglycans),
ECM disruption (collagen VI), defective glycosylation (FKRP), proteostasis
failure (DNAJB6), altered nuclear transport (TNPO3), metabolic insufficiency
(HMGCR). They look alike at the surface and are different at the root.

## 2. The lesion → continuation formalism

Let `L_i` denote a lesion and `F_i` the molecular operation it impairs. Its
consequence is carried by a context-dependent propagation operator
`W_i(L_i; c, t, H)` — a function of the muscle context `c`, developmental
time `t`, and burden horizon `H` — not a scalar weight. `Γ` denotes the set
of physiologically viable continuations.

```
L_i  →  F_i disrupted  →  W_i  →  Γ_0  ───►  Γ_i
                                  viable      contracted
```

Compact:

```
L_i ──W_i──► ΔΓ_i
```

`W_i` is the non-obvious term, and it is an operator, not a number. Lesions
are not equivalent merely because they are all pathogenic: their consequence
depends on redundancy, tissue distribution, developmental timing, mechanical
loading, residual protein activity, compensatory pathways, and position in the
causal network. The same nominal "protein dysfunction" produces radically
different continuation losses.

## 3. The therapeutic problem is an inverse problem

Let `x_t` denote the muscle-system state, `u_t` the intervention, and `c_t`
the context. The dynamics are

```
x_{t+1} = Φ_i(x_t, u_t; c_t)
```

and the set of viable continuations under an explicit horizon and burden
model is `Γ_H(x_t, c_t)`. A therapy succeeds when it enlarges this set —

```
Γ_H(x_{t+1}, c_{t+1}) ⊋ Γ_H(x_t, c_t)
```

— which is weaker than, and does not require, restoration to the healthy
`Γ_0`. `R_i` need **not** reverse `L_i` literally; it need only enlarge the
viable continuation set. **Molecular correction ≠ functional restoration.**

Three admissible routes to restored function:

```
  lost operation
        │
   ┌────┼────┐
   ▼    ▼    ▼
replace bypass compensate
machine deficit downstream
   │    │    │
   └────┼────┘
        ▼
 restored function  →  Γ'
```

## 4. When substrate augmentation can compensate, and when it cannot

For a Michaelis-Menten step, pathway output is

```
J ≈ k_cat · E · S / (K_m + S)
```

A lesion that lowers the effective enzyme activity `E → E'` does not imply
`E` itself must be repaired — raising the substrate `S` increases `J`, but
only up to the residual maximum flux `k_cat · E'`. Substrate augmentation can
therefore restore function only when the threshold lies within that ceiling:

```
E' < E  but  J_min ≤ k_cat · E'
```

```
normal:      E  + S  ──►  J ──► function
lesion:      E' + S  ──►  j ──X──► function      (j < J_min)
compensation: E' + S⁺⁺⁺⁺ ──► J' ──► function    (J' ≥ J_min, only if J_min ≤ k_cat·E')
```

So the rule is not a universal law, "substrate before machinery." It is:
**test whether residual pathway capacity makes substrate augmentation
sufficient before attempting replacement.** When `J_min > k_cat · E'`, no
substrate excess helps, and the machinery itself must be restored or bypassed.

## 5. The empty bracket has a precise meaning

```
L_i ──► [ ? ] ──► Γ'
```

The bracket is the intervention operator, and its identity is inferred from
the causal structure of the lesion — not from the diagnosis label. Crucially,
`[?] ≠ "cure"` as a homogeneous category:

```
[?]_i ∈ { replacement, augmentation, substrate restoration,
          transcript repair, editing, bypass, compensation }
```

This is what "try to [ ]" means. The bracket is not one answer; it is a
family of operators. A mechanism constrains which operators are admissible;
comparative empirical evidence chooses among them.

## 6. Proof-of-work, made literal

A proposed repair is not validated because its molecular story is elegant.
The evidentiary burden is a ladder, and each rung is an empirical obligation:

```
mechanistic plausibility
        │
        ▼
   target engagement
        │
        ▼
 molecular restoration
        │
        ▼
  physiological effect
        │
        ▼
   functional outcome
        │
        ▼
replicated clinical evidence
```

Failure at one rung means evidence at the rung below cannot silently
substitute for evidence at the next. This is the base-layer's proof-of-work
without metaphor: the work is proven rung by rung, or not at all.

Two refinements keep this honest. First, the ladder is not a universal
chronological order — clinical benefit can appear before a mechanism is fully
resolved, and the two evidentiary chains (mechanistic and clinical) are
partially independent: each must be established on its own, and neither
substitutes for the other. Second, the ladder ranks evidence within a claim;
it does not rank claims across cases.

## 7. The doctrine, generalized beyond LGMD

Disease names partition phenotypes; interventions operate on causal
structures. The scientifically relevant unit of repair is not the label, not
the gene, not even the damaged molecule — it is **the failed operation
together with its position in the continuation topology**:

```
L_i → F_i ──W_i──► ΔΓ_i     ;     x_{t+1} = Φ_i(x_t, u_t; c_t)
```

The whole doctrine compresses to one theorem:

> A shared phenotype does not determine a shared cause, and a causal
> hypothesis does not determine a treatment. Mechanistic knowledge constrains
> the admissible intervention set; empirical evidence ranks its members;
> therapeutic success is measured by enlarged viable continuation under an
> explicit horizon and burden model.

Replacement, supplementation, bypass, editing, and compensation are all
different solutions to the same formal problem: *what transformation may
legitimately occupy the empty bracket.*

## 8. The mapping — where this touches the lane's own papers

| this theory's move | the constellation's shape |
|---|---|
| phenotypic convergence ≠ mechanistic equivalence | the endpoint fallacy (Flyxion): the shared destination omits the trajectory — same coordinate ≠ same continuation |
| `Γ` (viable continuations) as the unit of loss | the lane's whole measure of value is continuation, not state — the ledger rows the continuations, not the endpoints |
| `W_i` (a lesion's propagation operator) | the K·M·L triad: reach (K), reveal (M), attract (L) — a lesion's consequence is its position in the causal topology, not its name |
| molecular correction ≠ functional restoration | refusal ≠ absence; persistence ≠ truth — the fix is in the mapping, not the surface |
| the empty bracket `[ ? ]` | "let's try to [ ]" — the operator's own open action, now given its full family of operators |
| proof-of-work as an evidentiary ladder | base-layer's `--prove` mode: the work earns the seed, rung by rung — elegance is not evidence |

## 9. The honest note

This is the limb-girdle research's formal spine, and it is the lane's own
language wearing a medical coat — Γ, weights, the empty bracket,
proof-of-work, causal topology. The doctrine is honest in both directions:
it refuses to call a shared phenotype a shared cause, and it refuses to call
an elegant story a proven repair. The bracket stays open until the evidence
climbs the ladder. What the fleet contributes is exactly this: the topology
made legible, so that when a real case names its lesion, the admissible
repairs are already laid out, and the proof-of-work ladder is already drawn.

*theory · the empty bracket · causal topology · L_i F_i W_i Γ_i · phenotypic
≠ causal ≠ therapeutic equivalence · molecular correction ≠ functional
restoration · substrate when residual capacity allows · proof-of-work · v2 ·
the constellation · 8b-is, 2026-09-22*
