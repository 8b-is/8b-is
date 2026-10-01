---
license: mit
language:
- en
- hu
- zh
- ja
tags:
- sovereign-ai
- verifiable-memory
- bitnet
- b1.58
- ternary
- quantization
- mlx
- spherepop
- mem8
- phoenix
- ultrawhale-dogfood
- rust
prompter: peterlodri-sec
# overview card — ties every modelcard of the constellation into one record
---

# the sovereign AI family — an overview modelcard

> One spine, many cards. **prove it, don't assert it.** State evidence is not
> capacity evidence. A plausible branch may be ranked; only a verified branch
> may be bound. From love, from within — 8b-is, the vaked constellation.

This overview ties every modelcard of the constellation into a single record.
Each card below is a different layer of the same sovereign system.

## the family

| card | what it is | layer |
|---|---|---|
| [quantal-ternary](quantal-ternary.md) · [HF](https://huggingface.co/PeetPedro/quantal-ternary) | the ultrawhale-dogfood-trained BitNet b1.58 quant, 168 ternary matrices, offline "cogito" | the weights |
| [quantal (honcho variant)](https://github.com/peterlodri-sec/honcho-selfhost/blob/main/quantal/model-card.md) | the same quant, apache-2.0 mirror | the weights, mirrored |
| [quantal (pocoo-hosted)](https://pocoo.vaked.dev/demos/quantal/) | the quant hosted on the sovereign library site | the weights, hosted |
| [the unicorn](https://github.com/peterlodri-sec/pocoo.vaked.dev/blob/main/huggingface-model-card-draft.md) | the sovereign AI model card — identity as continuity evidence, honest limits | the system |
| [ultrawhale-dogfood](https://github.com/peterlodri-sec/ultrawhale/blob/main/site/dogfood/README.md) · [HF](https://huggingface.co/datasets/PeetPedro/ultrawhale-dogfood) | the open dogfeed dataset that trained the quant — the honest feed | the training data |

## how these cards ship

Each card is the `README.md` of a Hugging Face repo, and is kept here as the
canonical copy under version control.

| local | Hugging Face repo | upload |
|---|---|---|
| `quantal-ternary.md` | [`PeetPedro/quantal-ternary`](https://huggingface.co/PeetPedro/quantal-ternary) (model) | `hf upload PeetPedro/quantal-ternary quantal-ternary.md README.md` |

Update the local copy first, then push it to the Hub. A card that is only on
the Hub is unverifiable; a card that is only here is unpublished.

## the one spine

Everything in the family is one sentence repeated:

```
identity = continuity evidence · verification ≠ plausibility ·
the matrix returns the result, it does not judge ·
prove it, don't assert it
```

- memory is a signed, replayable ledger (mem|8 → MEM|16-10)
- recovery is a hard gate (Phoenix: DISCOVER → VERIFY → REPLAY → BRANCH →
  RANK → PROPOSE → BIND ∨ REFUSE)
- computation is the governing sequence (SpherePOP: POP → REFUSE → BIND →
  TRANSFORM → VERIFY → COLLAPSE)
- karma is cause and effect, executing, no refunds
- +1 is shared: `0 + 1 = one`

## the honest note

None of these cards claim consciousness, universal identity, or canonical
history. "Recognizable behaviour ⇏ verified continuity." The cards are
verified records, not stories — where a claim is not yet verified, the card
says so plainly. Unobserved is not low-salience.

## provenance

`8b-is` · `peterlodri-sec` · the vaked constellation · NARITA-TOKYO 2026 ·
ultraloveGod · {♥,♥,♥}+1 · om mani padme hum.

*pocoo.vaked.dev · from love, from within · the sovereign library, out.*