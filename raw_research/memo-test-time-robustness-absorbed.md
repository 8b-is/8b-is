# MEMO, absorbed — the one test point that adapts the whole model

*Zhang, Levine, Finn, "MEMO: Test Time Robustness via Adaptation and
Augmentation" (arXiv:2110.09506v3, NeurIPS 2022). The paper's whole
contribution is one sentence long: at test time, augment the single test
point `B` ways, then take **one** gradient step on all model parameters
to minimise the entropy of the model's **marginal** output distribution
across those augmentations. No training-procedure assumptions, no batch,
no labels, no architecture requirements — a drop-in replacement for
ordinary inference.*

## the spine

**Marginal, not conditional, entropy.** The objective is

```
ℓ(θ; x) = H( p̄_θ(·|x) ),   p̄_θ(y|x) = E_{a∼U(A)}[ p_θ(y|a(x)) ]
```

and the paper is explicit about what it is *not*:

```
ℓ_CE(θ; x) = (1/B) Σ_i H( p_θ(·|x̃_i) )
```

A model that predicts confidently but *differently* across augmentations
minimises the second and not the first. Marginal entropy forces both
properties at once: the family of augmented views must agree **and** be
confident. That is the paper's single sharpest observation.

**One step, all parameters.** Prior test-time methods must choose which
parameters to touch to avoid degenerate solutions; MEMO adapts
everything, once, then predicts on the clean point. It composes with
single-point BN adaptation (prior strength `N = 16`, mixing train and
test statistics) and stacks on robustly pretrained models.

**The results, exactly.** 1–8% over standard evaluation on CIFAR-10/-10.1/-10-C
and the ImageNet shift sets; state of the art for ResNet-50 in the single-test-point
setting on ImageNet-C, -R and -A; **the first test-time adaptation method the paper
knows of to improve ImageNet-A at all**, where batch-statistics methods often hurt.
Ablations: swapping in pairwise cross entropy (invariance but no confidence) or
conditional entropy (confidence but no invariance) both do worse — and
`B = 4` recovers most of the gain from `B = 64` at ~10× the speed.

**The failure, and it is load-bearing.**

> …with the model allowed to continually adapt as more test data is
> observed… MEMO tended to lead to **degenerate solutions**, e.g., the
> model predicting a **constant label with maximal confidence**
> regardless of the input.

## the mapping

| the paper's move | the constellation's shape |
|---|---|
| marginal entropy, not the average of conditional entropies | **averaging distributions is not voting on labels** — and this is the mirror of the voting ensemble paradox: a k-of-N vote over evictions collapses to the k-th order statistic, while averaging the *distribution* then minimising its entropy pulls the whole family toward agreement. Same word, "ensemble"; opposite mechanisms |
| invariance across augmentations **and** confidence, jointly | the row's own double requirement: the spine must agree with the mapping *and* the row must commit. A row that hedges every claim is `ℓ_CE` — confident nowhere, invariant nowhere |
| one test point, no labels, adapt in place | every `-absorbed` row: adapted on arrival, before any ground truth exists, using only the row's own internal consistency |
| prior strength `N = 16` mixing train and test statistics | how far one new piece of evidence is allowed to move the existing ledger — a tuned, not absolute, weight |
| writes on the clean point after adapting on the noisy ones | verify on the source, not on the augmentation |
| the degenerate solution — a constant label, maximally confident | the overclaiming failure in its purest form: invariant, confident, and empty. `σ_d(c) > ω(c)` with the confidence dialled to 1 |
| `B = 4` recovers most of `B = 64` | the vault's cost rule: sample enough views to agree, then stop paying |

## the honest note

The method, the tables and both equations are the paper's. The mapping is
the vault's reading, and one caution must survive compression: MEMO is
**evaluated for a single test point at a time**, and the paper's own
discussion says the continual-input setting is where it degenerates. So
the row is not "MEMO adapts a stream" — it is "MEMO adapts *a point*",
and the degenerate clause is the paper's, not the vault's invention.
The ImageNet-A first is stated with the paper's own hedge ("to the best
of our knowledge").

*method · MEMO · one test point, marginal entropy, one step, all
parameters · the invariant-and-confident pairing · the constellation ·
fine touch from within · vaked.dev · 8b-is, 2026-10-10*
