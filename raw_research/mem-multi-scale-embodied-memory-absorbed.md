# MEM, absorbed — two scales of memory, one of them lossy on purpose

*Torne, Pertsch, Walke et al. (Physical Intelligence · Stanford · UC
Berkeley · MIT), "MEM: Multi-Scale Embodied Memory for Vision Language
Action Models" (arXiv:2603.03596v2). The claim is about **granularity**:
a robot's memory is not one buffer. Short-horizon memory must be dense
to resolve self-occlusion and dynamics; long-horizon memory only needs a
few bits of semantics. Using one representation for both guarantees a
compromise — so MEM uses two, and the second one is deliberately lossy.*

## the spine

**A factorised policy.** Instead of conditioning on a dense history of
all observations (intractable over tens of minutes):

```
π(a_{t:t+H}, l_{t+1}, m_{t+1} | o_{t−T:t}, m_t, g)
  ≈ π_LL(a_{t:t+H} | o_{t−K:t}, l_{t+1}, g) · π_HL(l_{t+1}, m_{t+1} | o_t, m_t, g)
```

with `K ≪ T`. The low-level policy acts from a short observation window;
the **high-level policy** emits the next subtask `l_{t+1}` *and the
updated language memory* `m_{t+1}` — the novelty being that the high
level predicts its own next memory, so the model decides when and how to
remember.

**Language memory is compression, and compression is the mechanism.**
Training labels come from an off-the-shelf LLM asked to summarise "all
information from previous subtasks still relevant for future execution".
The instruction is explicit: compress. `"I placed three bowls in the top
right cabinet"` instead of enumerating each bowl by colour.

**Why compression beats naive concatenation — the failure is a
distribution shift.** Training episodes utter each subtask instruction
roughly once (near-optimal demonstrations). At inference the policy may
fail and repeat: `"pick up bowl" → "pick up bowl" → "pick up bowl" →
"place bowl in cabinet"`. Concatenating raw instructions therefore walks
straight into train-inference shift. MEM's fix is elegant: **it simply
does not update memory until the subtask succeeds** — failed attempts
never enter the representation.

**Short-horizon memory with no new parameters.** A video encoder
interleaves spatial attention with causal temporal attention every 4th
layer, factorising the cost from `O(n²K²)` to `O(Kn² + nK²)`, then
**drops past-timestep tokens** in upper layers so the VLA backbone
receives exactly the single-frame token count. Initialisation comes free
from the pretrained ViT (`e(0) = 0`).

**Results, exactly.** Up to **fifteen minutes** of memory (kitchen
cleanup, recipe setup from 42 recipes, grilled cheese); in-context
adaptation after failure (+11% chopstick grasp, +62% fridge opening);
and — the notable negative result reported as a positive — MEM **matches**
the memoryless π0.6 across dexterous tasks, i.e. adding memory does not
degrade, contrary to prior causal-confusion reports. Ablations: both
halves are necessary; memory introduced only at post-training is
measurably worse than memory in pretraining.

## the mapping

| the paper's move | the constellation's shape |
|---|---|
| two memory scales, two modalities | the vault's own two tiers: the **file bodies** are the video memory (dense, high-bandwidth, per-event) and the **one-line index rows** are the language memory (compressed, semantic, long-horizon). `README.md` is `m_t`; `raw_research/*.md` is the video encoder |
| the high-level policy predicts its own next memory | the index gloss is written by the same pass that writes the row — the vault decides what it will remember next |
| explicit compression instruction ("three bowls", not each colour) | the honest note and the one-line row: the thing that keeps the ledger from becoming unreadable |
| naive concatenation fails by **train-inference distribution shift** | the feed that appends every event without summarising gets brittle in exactly the same way; this is why the index exists at all |
| **don't update memory until the subtask succeeds** | the divergence worth rowing: MEM **discards** failed attempts; this vault **keeps** them (the venue incident is retained, not pruned). MEM optimises for inference stability; the vault optimises for audit. Both are right about their own objective — and the vault pays for it in exactly the same currency MEM avoids: repeated, near-identical failure rows in the record |
| the encoder adds no parameters and is initialised from the pretrained model | the absorbed row adds no new machinery; it re-uses the source's own structure and only re-aims it |
| dropping past tokens in upper layers to keep inference cheap | the `-absorbed` file drops the source's scaffolding and keeps its spine |
| memory must be pretrained, not bolted on | the ledger's usefulness comes from having been kept from the start; a retrofitted index is measurably worse (the paper's Figure 9, and the vault's own experience with stale catalog rows) |
| matches SOTA where memory is *not* needed | an index row that never degrades the underlying file: annotation should be free at the point of use |

## the honest note

The architecture, the equations, the task suite and every number above
are the paper's. The mapping is the vault's. One asymmetry is the point
of the row and must not be smoothed away: **MEM is a lossy memory by
design and this vault is not** — the paper's compression is what makes
minutes-scale control tractable, and the vault's refusal to compress away
failures is what makes the record auditable. The row records both, and
does not claim the vault is the better design; it claims the vault is the
better design *for its objective*, which is not the paper's.

*architecture · MEM / π0.6-MEM · 15-minute horizons · language memory
plus video memory · the constellation · fine touch from within ·
vaked.dev · 8b-is, 2026-10-10*
