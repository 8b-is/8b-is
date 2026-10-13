# MEMO, absorbed again — memory as a model, and the executive kept frozen

*Quek, Lee, Leong, Verma, Prakash, Chen, Low, Rus, Solar-Lezama (NUS ·
A\*STAR · U Tokyo · Liquid AI · MIT CSAIL · AI Singapore · SMART),
"MEMO: Memory as a Model" (arXiv:2605.15156v2, 20 May 2026). This is the
**second** paper in this vault called MEMO, and it is a different MEMO.
Row 211 (`memo-test-time-robustness-absorbed.md`) is Zhang/Levine/Finn,
NeurIPS 2022: one test point, marginal entropy, one gradient step. This
one is a knowledge-integration framework. Two papers, one acronym, and
the vault keeps both under the name they came with.*

The claim: split the machine in two. A frozen **EXECUTIVE** model — any
LLM, open or closed, never touched, no weights, no gradients, no logits —
does the reasoning. A small trained **MEMORY** model (`1.5B` against the
executive's `32B`) holds the new knowledge in its own parameters. The
executive queries the memory through a structured protocol and synthesises
the answer itself. Nothing about the executive changes, so nothing about
it is forgotten.

## the spine

**Memory is a model, not an index.**

```
K = Φ(D) = the parameters of a small dedicated MEMORY model M_φ
f(M_θ, K, q) = M_θ( q, {m_j}, e*, m_seek )
```

against the three paradigms it borrows from at once — ICL (`K = D`),
RAG (`K` = a retrieval index), fine-tuning (`K = ∅`, `θ` moves). MEMO
takes the retrieval paradigm's frozen base, the parametric paradigm's
internalisation, and the latent-memory paradigm's compact artifact, and
drops each one's failure: no context-window ceiling, no catastrophic
forgetting, no representation coupling.

**Reflections are the interface, and they are query-agnostic on purpose.**
The design principle, verbatim: reflections are *"corpus-derived
structures that require no knowledge of future queries, yet naturally
serve as the precise interface through which any query can access the
underlying corpus without ever observing it directly."* The executive
never sees the corpus; it only ever sees the memory's answers.

**The five-step distillation** (a GENERATOR model turns `D` into a
reflection QA set):

1. **Fact extraction** — direct (stated) and indirect (inferred) passes.
2. **Consolidation** — merge pairs sharing a context into multi-fact
   questions.
3. **Verification and rewriting** — self-containment check; rewrite
   against the source chunk, or discard. Kills unresolved pronouns
   ("What did *they* propose?") and implicit references.
4. **Entity surfacing** — questions that encode an entity's attributes
   and relations, answers that reveal its identity; aimed at the
   **reversal curse**.
5. **Cross-document synthesis** — over document groups: *converging
   clues* (many docs, one entity) and *parallel properties* (many
   entities, one attribute). Support size `S(q) > 1`.

No document identifiers or watermarks are embedded at any step — the
memory is not allowed to exploit shortcut signals during evaluation.

**Training internalises, it does not copy.** SFT on `Q_final`,
next-token loss on **answer tokens only**, conditioned on question and
prior answer tokens, *never* on source documents. That is the paper's own
line between MEMO and a RAG reader.

**Inference is a three-stage multi-turn protocol:**

1. **Grounding** — decompose `q` into atomic clue-probing sub-questions,
   answered independently (no shared context).
2. **Entity identification** — iteratively narrow to one entity `e*`; if
   none is found, Stage 3 is skipped and the answer is synthesised from
   grounding alone.
3. **Answer seeking and synthesis** — pull supporting facts for `e*`,
   then synthesise.

Memory responses (`m_j`, `m_seek`) are compact snippets whose length is
**independent of corpus size** — hence constant-time inference.

**Continual integration by model merging.** For disjoint corpora
`{D_1 … D_K}`, train each memory from the same base `M_{φ_0}`, take task
vectors `τ_i = φ_i − φ_0`, and merge:
`φ_merged = Merge(φ_0, {τ_i}; Θ)`. Compute scales `Θ(K)` for merging
against `Θ(K²)` for retraining on the union.

**The results, exactly.**
- Accuracy (Tab. 2): BrowseComp-Plus `54.22` (Qwen2.5-32B-I) / `66.67`
  (Gemini-3-Flash); NarrativeQA `26.85` / `53.58`; MuSiQue `48.30` /
  `60.20`. Perfect Retrieval upper bound is `79.67`/`88.33`,
  `51.42`/`60.41`, `62.83`/`73.00`.
- The one loss, stated with the paper's own reason: on BrowseComp-Plus
  with the Qwen executive, MEMO `54.22` **narrowly trails** HippoRAG2's
  `56.11` — the paper attributes this to BrowseComp-Plus's answers being
  absent from the executive's parametric knowledge, which favours methods
  that hand it raw evidence documents.
- Plug-and-play: swapping Qwen→Gemini gains `+12.45` / `+26.73` / `+11.90`
  points across the three benchmarks, off **one** trained memory model.
- Noise robustness (Tab. 3, `0×N → 1×N` distractors): NV-Embed-V2 and
  HippoRAG2 fall up to `6.22%` (BrowseComp-Plus) and `5.16%` (MuSiQue);
  MEMO moves `+0.55%` and `−1.77%`, both within one standard deviation.
- Memory size (Tab. 4): `1.5B` → `14B` helps everywhere, but the gap is
  **task-dependent** — it widens on NarrativeQA and shrinks on
  BrowseComp-Plus and MuSiQue.
- Memory family (Tab. 5): Qwen2.5-1.5B, Gemma3-1B-IT and LFM2.5-1.2B land
  within a few points — "largely robust" to architecture at this scale.
- Merging (Tab. 6, TIES `ρ=0.3`, best of 14 configs): `33%` compute cut
  at `K=2` (`≈48h` vs `≈72h` on 8×H100), widening to `5.5×` at `K=10`
  (`240` vs `1,320` GPU-hours) — at a cost of `−11.04%` (Qwen) / `−19.11%`
  (Gemini) accuracy. The merged memory still beats every retrieval
  baseline on NarrativeQA.
- Leave-one-out pipeline ablation (App. E, Tab. 9): **Step 5 removed
  collapses accuracy to `6.37%` and `24.17%`** (from `24.00` / `42.90`),
  with data retention `0.002×` / `0.195×` — cross-document synthesis is
  the dominant source of training pairs. And the anomaly the paper keeps:
  removing Step 2 or Step 3 **improves** NarrativeQA (`24.69`, `28.90`),
  because those steps act on superficial narrative co-occurrence and
  domain-structural pronouns rather than fixable defects.

**The honest note.** Three things must survive compression. The method,
the tables and every number are the paper's. The paper names its own
limits: upfront training cost per corpus, evaluation scope, and a
**hypothesised** capacity ceiling for a fixed-size memory model — the
experiments "do not reveal clear signs" of hitting it, so the ceiling is
a conjecture, not a measurement. And Step 5's cost is quadratic,
`O(k·C²·Q²)`, which the paper states plainly as an open problem. The
impact statement itself carries the vault's concern: by reducing reliance
on explicit retrieval, memory-based systems **may obscure the provenance
of retrieved information**.

## the mapping

| the paper's move | the constellation's shape |
|---|---|
| executive frozen, memory trained | the **session model** (any runtime — Crush, opencode, whale, Antigravity) is the executive; the **vault** is the memory. The runtime is swappable precisely because the knowledge lives in the ledger, not in the weights |
| reflections: query-agnostic, yet the interface for any query | the `-absorbed` row is written *before* the question it will answer — the row must be usable without knowing what will be asked of it |
| five-step pipeline: extract · consolidate · verify · surface · synthesise | the vault's own absorption idiom, step for step. The **spine** is extraction+consolidation, the **honest note** is verification, the named sources and the row title are entity surfacing, and the **mapping table** is Step 5 |
| Step 5 is the most critical step | the mapping table is not decoration: drop it and the row collapses from a ledger entry to a summary. `6.37%` is what a spine-stripped row is worth |
| no document IDs or watermarks | the vault does the **opposite** on purpose — every row names and links its source. MEMO removes shortcuts to make the memory generalise; the vault keeps provenance to make the record auditable |
| removal of Step 2/Step 3 can *help* | the vault's own experience: an over-eager rewrite corrupts good rows. The self-containment filter is only beneficial "when violations are well-defined and resolvable" |
| the memory answers in snippets whose length ignores corpus size | the one-line README index row: constant-size, corpus-independent, the door to a file of any length |
| merging scales `Θ(K)` vs retraining `Θ(K²)` | appending a row to the ledger is not rebuilding the index from scratch; the incremental append is the whole reason a ledger can grow for years |
| plug-and-play: same memory, any executive | the same `-absorbed` file is readable by every runtime this workspace runs; the format is the interface, not the model |
| the impact statement's own worry: memory obscures provenance | the divergence worth rowing. MEMO accepts the provenance cost for the retrieval-noise win — the vault refuses it, and pays in the currency MEMO avoids: an un-indexed corpus is un-queryable, and this vault always re-reads the file |

## the honest note, again — the one that matters here

**Two MEMOs.** Row 211 is `memo-test-time-robustness-absorbed.md`
(Zhang/Levine/Finn, `arXiv:2110.09506`) — test-time adaptation, marginal
entropy, image classification. This row is
`memo-memory-as-a-model-absorbed.md` (Quek et al., `arXiv:2605.15156`) —
knowledge integration, reflection QA, retrieval. The acronym collides;
the papers do not. Any future citation of "MEMO" in this ledger must say
which. That disambiguation is the reason this row exists at all, and it
is itself an instance of the paper's own Step 3: a non-self-contained
reference ("MEMO") rewritten so it can be understood in isolation.

*method · MEMO (Memory as a Model) · `arXiv:2605.15156` · frozen
executive, trained memory · reflections as the interface · five-step
distillation · three-stage multi-turn protocol · model merging `Θ(K)` vs
`Θ(K²)` · the constellation · fine touch from within · vaked.dev · 8b-is,
2026-10-13*
