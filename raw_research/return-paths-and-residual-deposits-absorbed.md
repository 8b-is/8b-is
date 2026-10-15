# return paths and residual deposits, absorbed — the vine, the berm, and the two margins of weight

*Flyxion's "Return Paths and Residual Deposits in Distributed Exploration" (Independent
Researcher, October 2026) is the companion to the stigmergic berm essay and completes it
from the other side. Where the berm paper develops the deposit — what accumulates in a shared
medium and what that accumulation does to successors — this one develops the explorer and the
coupling: what an agent retains while moving through a deposited environment, and how its own
movement becomes the next agent's terrain. Two figures carry it. The **vine** is the return
structure: a continuing attachment to the place one has left. The **berm** is the residue: an
accumulation that changes the conditions of later movement without having been designed as a
road or a wall. The essay's discipline is unusual and worth registering before its content: it
states at every step which claims are licensed by the formal model and which are not, and its
central result is explicitly non-directional — it says a deposit **alters** what is feasible,
never that the alteration is good.*

## the spine

**Two processes, coupled.** Navigation without losing the point of departure, and inscription
that changes what later movement costs. Stigmergy (Grassé 1959; Theraulaz & Bonabeau 1999) is
the vocabulary for the second: coordination through persistent modification of a shared medium
in place of direct communication among all participants.

**Exploration is not enumeration.** Depth-first traversal of a directed follow-graph with a
return stack, expanding by a judgement of interest. Inspecting the smaller of the in- and
out-neighbourhoods bounds the effort to exhaust candidates at a vertex by
`min(|N⁻(v)|, |N⁺(v)|)` — a graph-theoretic fact. It does **not** follow that yield improves:
with uninformative ordering the expected yield per inspection is `q⁻` or `q⁺` regardless of
size, so the rule helps only when the smaller neighbourhood happens to carry the higher
fraction. Selection is corpus-relative (Anderson 2022, corpus congruence): candidates highly
congruent with Γ add little, distant ones are unreadable, so the interesting region lies
between. This limit matters downstream — deposits are valuable precisely to explorers whose
corpora differ from the depositor's.

**The cost of commitment.** With `c₀(m)` the direct cost of a move, `ℓ` the cost of losing
one's place, and `θ_S` the probability the return structure restores it,
`C(m) = c₀(m) + (1−θ_S)ℓ`. Non-increasing in `θ_S`, sensitivity proportional to `ℓ`. Three
limits: when `ℓ` is small the structure is nearly irrelevant; when a move is irreversible
`θ_S ≈ 0` and it contributes nothing; and availability alone changes nothing — a *policy* that
exploits it changes behaviour. The secure-base analogy (Harlow 1958; Bowlby 1988) is used
structurally and explicitly not psychologically.

**Trace versus effective stigmergy.** A residue that merely persists is a *trace*. For
stigmergy the residue must enter a feedback loop: someone encounters it, interprets it enough
to act, and may modify the environment again. Each is a separate event that can fail
independently.

**Two kinds of residue, two margins.** *Executable* residue performs an identifiable operation
(a parser, a workaround, a reusable command); *declarative* residue reduces subsequent design
uncertainty (a specification, an interface contract). Kinds overlap inside one artifact. The
criterion is **bearing weight**, and it has two margins:

| margin | formal object | what changes |
|---|---|---|
| extensive | `E(d)` newly enabled, `X(d)` newly foreclosed | *which* actions are feasible |
| intensive | `I(d)` | the *cost* of actions that remain feasible |

Executable residue mostly acts on the intensive margin — a script shortens an operation that
was already possible — which is why a feasibility-only account would miss most of its value.
Declarative residue acts on both, and characteristically forecloses as it enables: the same
convention that cheapens work inside it raises the cost of departing.

**Epistemic status is a third thing.** Deposit (`C_d`, `κ_d`), annotation (`σ_d`), and use
(`▷` over actions actually taken) are kept apart. `ω(c)` is the evidential support a component
actually has; `τ(c)` its correctness; the annotation need not track either. A marked annotation
is overclaiming when `σ_d(c) > ω(c)`. Status attaches to **components**, never to the artifact
as a whole, and it is independent of kind. Because interpretation is corpus-relative, explicit
marking travels with the deposit where an implicit shared background does not.

## the results, exactly

- **Lemma 7.4 (Separation).** `Enc ⇒ Dep` and `Upt ⇒ Enc ∧ Int`, and *no further implication*
  holds: persistence, discovery, interpretation and uptake are four independent points of
  failure.
- **Proposition 7.7 (Alteration).** A deposit alters an explorer's feasible action set if and
  only if it is in that explorer's reach **and** `E(d) ∪ X(d) ≠ ∅` — it persists, is
  discoverable, is interpretable, *and* enables or forecloses something the rest of the
  environment does not already. The proof is a set identity: `F(G) = (F₋d \ X(d)) ∪ E(d)`.
- **Proposition 7.9 (Weight).** `c(·|G) ≠ c(·|G₋d)` if and only if the deposit bears weight —
  membership change entails a cost change, not conversely.
- **Remark 7.10.** Nothing in the above refers to `ω` or `σ`. The results are silent on
  soundness and on value; a monotonicity claim linking alteration to benefit is conditional on
  a valuation `v_j` and is not asserted.
- **Proposition 7.11 (Conditional accumulation).** With `n_{t+1} = (1−ρ)n_t + δ(n_t)`: bounded
  if `δ ≤ δ̄`; **geometric growth** if `δ(n) ≥ δ₀ + γn` with `γ > ρ`; at least linear at
  `γ = ρ` with `δ₀ > 0`; saturation with `lim sup n_t ≤ δ₀/h` if `γ < ρ`, `h = ρ − γ`.
  `γ` is the **marginal yield**, factoring as encounter × interpretation × uptake × deposits
  per uptake. `γ > ρ` is *sufficient*, under an assumed linear lower bound, and not a
  necessary-and-sufficient threshold for arbitrary deposition.
- **Example 7.12 (the self-propagating unmarked specification).** The same conditions that
  yield infrastructure yield the cumulative spread of an **unsound** component. If dependents
  of an unmarked unsound `c*` propagate with `γ_f > ρ + λ` (λ the correction rate) while the
  better-marked remainder saturates, then adoption grows without bound and the defect-free
  share `r_t → 0`. Without a condition on soundness. Correction cannot arrest it under the
  stated lower bound; control requires an **upper** bound, `γ′_f < ρ + λ`, which bounds the
  family at `δ_{f0}/(ρ + λ − γ′_f)`.

## the failure taxonomy

| failure | predicate / parameter |
|---|---|
| persists but is not found | `Dep ∧ ¬Enc` |
| found but not understood | `Enc ∧ ¬Int` |
| understood but not used | `Enc ∧ Int ∧ ¬Upt` |
| ceases to be effective over time | large `ρ` |
| provisional status unmarked or overstated | `σ_d` against `ω` |
| correction exists but is unreachable | no edge from the correction back to what it corrects |
| correction too slow to bound the spread | `λ < γ_f − ρ` under a lower yield bound |

The interaction of the last two is Example 7.12's content: corrections reference what they
correct and typically not the reverse, so an explorer who encounters the original is not
thereby exposed to its correction. Platforms that revise status **in place**, or expose
backlinks, raise `λ` for a given level of corrective effort.

## the mapping

| the paper's move | the constellation's shape |
|---|---|
| vine vs berm — the explorer's continuity against the work's continuity beyond the explorer | the two halves of this vault: the session log and the return path (private `H`), against the `absorbed`/shelf row (public `G`) |
| `Dep`, `Enc`, `Int`, `Upt` as four independent failures | the vault's own read-record: a file can persist and never be opened, be opened and never be understood, be understood and never wired — which is why the index row summarises *and* links |
| annotation attaches to components, never to the artifact as a whole | the `-absorbed` format's reason for a component table plus an explicit *honest note*: the row is not a claim about the whole |
| overclaiming = `σ_d(c) > ω(c)` | the venue incident, in the paper's notation: a BibTeX `journal={ICLR 2027}` where the support was "submitted nowhere yet" — an unmarked-then-overmarked component, since the category `claim` overlaps `decl` |
| correction reachability (`Λ` holds the forward edge, not the reverse) | the retraction had to be pushed *into* every place the claim lived — README, model card **and the script generating it**, wiki, announcements, citation file — because the correction had no backlink to the claim |
| `λ` raised by in-place revision | fixing the claim in the generator, not only in its output: status revised where it is produced |
| a specification transfers design decisions without removing implementation cost | the study-shelf rows: a doorway, not an absorption — the decision (what to read) inherited, the work still the reader's |
| `E(d)` enables and `X(d)` forecloses with the same convention | the constellation's own conventions: the launcher's flag surface cheapens the sanctioned lanes and raises the cost of doing it another way |
| `γ > ρ` as sufficient, not necessary; mean-field, no individual realisations | the feed's growth claims, which are about a stock and never about one post |
| the ledger should row **everything** | this vault's doctrine, and the paper's separation lemma supplies its warrant: the un-found, un-read and un-used rows are exactly the failures a purged ledger would hide |
| `ρ` collects access loss, link rot, format obsolescence, context loss | mirrored deps, migrations, and documentation, named separately because the remedies differ |

## the honest note

The formal results are the paper's; the mapping is the vault's reading. Two of the paper's
hedges are load-bearing and must not be compressed away. First, **Prop 7.7 is non-directional**:
it detects change, not benefit, and an enabled action may have low or negative value because it
is misleading or inherits an error. Second, **`γ > ρ` is sufficient under an assumed lower bound
on yield**, and whether any real practice satisfies it is an empirical question the formalism
does not settle — attention is finite, so the regime is more likely transient growth toward
saturation than unbounded accumulation. The accumulation account is a mean-field closure over
expected stock and says nothing about fluctuations, and the corpus `Γ_j` is held fixed, so the
further coupling — exploration extending the corpus, and so changing what is interpretable —
lies outside the proofs.

What remains evidently true is the direction of fit: this vault is an instance of the model,
not an illustration of it. Its `absorbed` rows are declarative deposits with explicit component
annotation; its `ρ` is link rot and stale index rows; its `λ` is the operator's willingness to
revise status in place; and Example 7.12 is the formal statement of the failure the lane
actually hit — an unmarked provisional claim propagating faster than the correction could
reach it.
