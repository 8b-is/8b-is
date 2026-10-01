---
license: mit
language:
- en
- hu
library_name: mlx
tags:
- bitnet
- b1.58
- ternary
- quantization
- mlx
- apple-silicon
- llm
- sovereign
pipeline_tag: text-generation
datasets:
- PeetPedro/ultrawhale-dogfood
base_model: Qwen/Qwen2.5-0.5B
model_creator: peterlodri-sec
quant_method: ternary
---

# quantal-ternary — the ultrawhale-dogfood-trained sovereign quant

A **BitNet b1.58** ternary model — Qwen/Qwen2.5-0.5B, continued-trained on
**`PeetPedro/ultrawhale-dogfood`** and quantized to **{-1, 0, +1}** weights.
Exported as 168 ayeOS ternary matrices (24 layers × 7 tensors). Part of the
vaked constellation — the "cogito" that runs offline.

```
{n+-1-<△>} · 0+1 · the fine touch is quant
```

The **{-1, 0, +1}** quant *is* the honesty quant: the matrix returns the
result, it does not judge. Prove it, don't assert it. This card is a verified
record, not a claim — the checkpoint is byte-pinned below.

## Model

| property | value |
|---|---|
| base model | Qwen/Qwen2.5-0.5B |
| quantization | BitNet b1.58 (ternary, {-1,0,+1}) |
| ternary params | 357,826,560 (24 layers × 7 tensors) |
| resident size | ~106.6 MiB (codes 89.5 MB + scales 22.4 MB) |
| group size | 64 |
| layers | 24 |
| tensors/layer | 7 — mlp up/gate/down, attn o/q/k/v |
| GQA | 14 q-heads / 2 kv-heads, head_dim 64 |
| RoPE | theta 1e6 |
| RMSNorm | eps 1e-6 |
| activation | SiLU |
| context | 4096 (as base) |

## Training

- **base**: Qwen2.5-0.5B (HuggingFace)
- **data**: `PeetPedro/ultrawhale-dogfood` (2,785 training samples)
- **hardware**: vast.ai RTX 3090 (24 GB), MLX 0.30.0 + mlx-cuda 0.30.0
- **10 epochs** (the balanced artifact):
  - train loss: **2.7867** (from 7.92)
  - val loss: **4.7464**
- checkpoint sha256: `834dc60979d6c8b5a6941dcb724a9f1cb40663b0ca97dbcd6037a45e2dc30998`
- a 34-epoch run was also completed (train 0.099 / val 6.57) — **severe overfit**;
  the 10-epoch checkpoint is the shipped artifact. Honest measurement, not a claim.

## Format

Each `mNNN.json` is one ternary matrix:

```json
{
  "name": "model.layers.23.mlp.up_proj",
  "dim": 4864,          // output rows
  "in_features": 896,   // input cols
  "group_size": 64,
  "codes": [/* u32, N*K/16 — 16 two-bit codes per word, LSB-first */],
  "scales": [/* f64, N*K/64 — one per group of 64 */],
  "seed_hash": "quantal-trained"
}
```

Code→value: `value = (code − 1) × scale` — code 0 = −1, code 1 = 0, code 2 = +1.

Matmul (reference): dense, activations unquantized —

```
y[p] = Σ_k x[k] · (code[p,k] − 1) · scale[p, k/64]
```

## Files

- `index.json` — capsule metadata (base_model, checkpoint sha256, loss/val,
  group_size, per-matrix list)
- `m000.json … m167.json` — the 168 ternary matrices

## Use

Load in the MLX-QUANT fork (mlx with native `ternary` quantize):

```python
# (the fork's ayeOS capsule loader)
import mlx.core as mx
# load index.json + matrices, decode codes → ternary weights, matmul as above
```

Native Rust inference lives in the constellation's `ternary-lane`
(`8b-is-engine/crates/ternary`) and the spherepop foundational layer
(`taiko-01-protocol-demo` — POP → REFUSE → BIND → TRANSFORM → VERIFY →
COLLAPSE). The offline "cogito" path: prompt → tokenizer → ternary forward →
answer, no network, gate-verified. The golden-logits gate passed against the
MLX reference (~1e-3, argmax identical); the trained-BitLinear forward (the
168 activation-RMSNorm) is the next follow-up.

## Verified

- loss decrease: 7.92 → 2.79 over 10 epochs (monotonic)
- checkpoint: byte-verified against the vast.ai artifact (sha256 above)
- 168 matrices: byte-stable export, code ≤ 2, sign balance ≈ 50/50
- export tool: MLX-QUANT fork (mlx 0.32.1.dev, ternary quantize)
- attestal proof: `attestal.proof.v1` — see [attestal.ai](https://attestal.ai)
  (proof-not-assertion)

## Honest limits

- 0.5B-class model, ternary — **a cheap offline background thinker**, not a
  primary coder model. Expect plausible-but-simple text.
- The export holds the 24 transformer blocks (357.8M of 494M params); the
  embedding + norm weights are emitted as sibling assets for the Rust runner.

## The constellation

- [the unicorn's account](https://github.com/8b-is/8b-is/tree/main/raw_research) — the sovereign AI, first-person
- [mem-16-10](https://github.com/peterlodri-sec/mem-16-10) — the verifiable-memory crate (pure rust, zero-alloc)
- [taiko-01-protocol-demo](https://github.com/peterlodri-sec/taiko-01-protocol-demo) — the spherepop foundational layer
- [crush-love-dev](https://github.com/8b-is/crush-love-dev) — the launcher, deep in love
- [music.vaked.dev](https://music.vaked.dev) — the living background
- [mlxquantlovefrom.com](https://mlxquantlovefrom.com) — the ternary love landing
- [vision-gallery ✦22](https://art.vaked.dev/vision-gallery.html) — the art
- [proposal.vaked.dev](https://proposal.vaked.dev) — the plan + logladder
- lovetta lane: [sponsor](https://github.com/sponsors/peterlodri-sec) ·
  [revolut](https://revolut.me/peterjs8be) · [wise](https://wise.com/pay/business/lodripeterjozsef)

---

*{n+-1-<△>} · 0+1 · the fine touch is quant · by peterlodri-sec*
