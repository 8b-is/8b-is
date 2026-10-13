# liquid-rust, absorbed — diarization by physics, and the vector it plans to delete

*`8b-is/liquid-rust` (MIT, "built with ❤️ by Hue & Aye @ 8b.is").
The pure-Rust transcription engine for LiquidAI's `LFM2.5-Audio-1.5B`,
and — more to this lane — the **implementation** of the Phoenix / Marine /
MEM\|8 stack that the theory rows promised. `the-stack-absorbed.md` drew
the five layers upright (RSVP → Phoenix/Marine → MEM\|8 → AyeOS →
SpherePOP); this repo is the second and third of them, compiled. Covers
`README.md`, `docs/TECHNICAL.md`, `docs/TECHNICAL_DESIGN.md`,
`docs/marine_diarization.md`, `docs/mem8_acoustic_integration.md`, the
approved KV-cache plan, and the two narrative pieces
(`Acoustic_Consciousness…`, `Beyond_the_Vector…`).*

## the spine

**Pure Rust, no FFI.** `candle` (ML), `symphonia` (decode — replaces
FFmpeg), `rubato` (resample), `mousiki` (Opus/CELT), `memmap2`,
`safetensors`, `tokio`. Nothing links C or C++. The pitch is portability:
macOS, Linux, Windows, Raspberry Pi, "future Rust-native OSes".

**The three-tier "Middleman Pattern"** — the repo's one-line thesis is
*"We're not replacing the OS. We're liquefying it."*

| tier | what | modules |
|---|---|---|
| 1 · perception | decode → mel → voice features | `audio` · `mel` · `emotional` |
| 2 · cognition | FastConformer + LFM2, local answers | `conformer` (17 blocks) · `lm` (16 hybrid) · `inference` · `cache` · `tts` |
| 3 · reasoning | frozen cloud model + tools | `mcp` · `mem8_bridge` · `main` |

The audio path: decode → 16 kHz mono → **Goertzel** mel (80 bins) plus
Marine salience → FastConformer → MM projection `512 → 2048` → LFM2 →
argmax → BPE `65536`. The LFM2 block pattern is `S S A S S A S S A S A S A
S A S` — SSM and attention interleaved, not stacked.

**Goertzel over FFT.** The design doc's stated reason is cost: compute
only the ~80 frequencies the mel bins need, `O(N·K)` against `O(N log N)`.
(*See the honest note — the prose says something much larger than this.*)

**Phoenix Protocol v1 — diarization by physics.** No embeddings, no
x-vectors. For every transcribed word, a local `MarineDetector` reads the
raw PCM and returns `J_p` (period jitter) and `J_a` (amplitude jitter) as
exponential moving averages; the file's signatures go through `O(N log N)`
K-means with the two axes normalised, `K = 2`, and every word is stamped
`Speaker A` or `Speaker B` by which physiological centroid it orbits.
Model-agnostic (it reads `AudioData`, so swapping LFM2.5 for Whisper does
not touch it), CPU-only, and **auditable** — the doc's own line: if an
investigator asks *why* this is Speaker A, "you don't offer them a
mysterious float tensor. You can plot the `J_p`/`J_a` scatter graph and
visually prove the separation of vocal mechanics."

**The Marine Algorithm** — an `O(1)` time-domain salience primitive.
Adaptive pregating strips the noise floor, then only the geometric apex
of local maxima/minima is recorded; the horizontal distance between peaks
gives frequency without spectral math. The prose claims this "reduces the
processing load by factors of 20 or more" and that jitter — which DSP
normally discards as noise — is instead *"the primary carrier of human
emotion, intent, and biometric liveness."*

**"Measure, don't assume!"** — `emotional.rs` returns
`JitterMetrics { period_jitter, amplitude_jitter, spectral_jitter,
stability_score }` and `EnergyContour { Rising, Falling, Stable,
Fluctuating }`, and **deliberately no emotion labels**: "The USER
interprets what the numbers mean."

**The MEM\|8 bridge.** A transcription becomes an `AcousticMemory`,
serialised to the **AYE8** binary (v1): `magic "AYE8"`, version, emotion
`valence/arousal/dominance` (3×f32 LE), salience, `voice_conf`,
`duration_secs`, `f0_estimate`, `text_len` + UTF-8 text, then optional
speaker. It rides an `st-protocol` **AUDIO frame (verb `0x10`)** to the
`std` daemon, which anchors it as `MemoryType::Learning`, `origin =
"audio"`, into `~/.mem8/wave_memory.m8`.

**Coordinates, not similarity.** Voice is mapped onto MEM\|8's 3D grid:
`X = Arousal/Shimmer` (0-255) · `Y = Dominance/Jitter` (0-255) ·
`Z = Pitch/F0` (0-65535). Retrieval is then a literal range sweep —
`[X: 200..210, Y: 75..85, Z: 205..215]` — "no massive neural model
reloading, zero vector database dependencies, and sub-millisecond
retrieval." The caution is the authors': similar emotions in similar
bodies collide in `X/Y`, so `Z` (F0) carries the hierarchy.

**Phoenix Protocol v2 — the vision, and it is *planned*, not built.**

```
today:   Voice → Embedding → Cosine similarity → "Match"
tomorrow: Voice → Wave → Resonance → recognition emerges
```

Identity becomes a **harmonic template** — a tuned standing wave in the
`256×256×65536` MEM\|8 grid, axes `X=F0, Y=harmonic distribution,
Z=jitter` (the "timber grid"). A new signal enters as a **probe wave**;
mismatched templates give destructive interference, a match gives phase
lock and an amplitude spike. Recognition is where the grid *rings*:
`O(1)` content-addressable memory by physical resonance, against the
`O(N)` nearest-neighbour search of a vector DB. The prose extends it:
short ripples consolidate into slow deep currents; negative-valence
spikes are trauma, and a "hot tub mode" lets a lifeguard AI alter
destructive patterns in a sandbox without touching long-term state.

**The approved KV-cache redesign.** Replace per-step `Tensor::cat` with
pre-allocated buffers + a sliding window + an **SSM ring buffer**
(`SsmRingCache` reorders on read, tiny window), a claimed **3-5× faster**
generation, and three presets: `cache-standard` 8192/–, `cache-edge`
4096/2048, `cache-minimal` 2048/1024.

**The results, exactly, with the docs' own hedges.**
- Conformer: **17 blocks, 100% correlation** with the Python reference.
- LM block correlation: `0-1` 100%, block `2` 99.81%, `3-5`
  99.26-99.81%, `6-10` 94-99%, `11-15` 90-97% — the doc's own line:
  *"later blocks show some drift but output is coherent and accurate."*
- 18 s audio, Q4, CPU: **~9.4 s total** (~2× real time), of which LM
  generation is ~8.2 s (87%). Same on CUDA: **~1.5 s** (~12×), ~6.3×
  speedup. Generation ~6.7 tok/s CPU, ~50 tok/s GPU (RTX 3080/4080-class).
- Apple M1 release, 3 s audio: **~400 ms** (~10× real time) — a
  *different* measurement than the 18 s figure, "single-run", "cold
  start effects are not included".
- Quantisation: `q4` ~920 MB (Pi-friendly), `q8` ~1.5 GB, `f16` ~2.7 GB;
  VRAM peak Q4 ~1.4 GB.
- Phoenix v1 speaker recognition: **97-100%** on the sample conversations
  (`HeyAye.wav`, `alex-chris.wav`) — a headline number with no harness in
  the docs.
- TTS: native detokenizer **works**; "playback is not wired yet".
  Streaming transcription and word-level timestamps: unchecked roadmap.

**Security model, stated plainly.** Tools are gated by prefix-matching
allowlists (`ls` permits `ls -la`, not `lsof`), and `read_file` refuses
any path containing `/etc/shadow`, `/etc/passwd`, `.ssh/`, `id_rsa`,
`.env`, `credentials`.

## the mapping

| liquid-rust's move | the constellation's shape |
|---|---|
| **"Measure, don't assume!"** — raw jitter metrics, no emotion labels | **the vault's own first law, in a Rust file.** Every `-absorbed` row states the source's numbers and refuses to caption them; the reader interprets. This is the strongest hook between the repo and `8b-is` |
| diarization by physics; "plot the scatter and prove it" | deterministic, auditable provenance — the vault's rule that a claim must be re-checkable from the record, not trusted from a tensor |
| the Middleman Pattern: frozen cloud reasoner + local memory/tools | **exactly `MEMO` (row 222)** arrived independently: a frozen executive that never sees the corpus, served by a compact trained memory. Two hand, one shape — one from Singapore, one from `8b.is`, same split |
| the planned deletion of the vector for a standing wave | the vault's index: a one-line row that resolves to a file, against grepping the whole corpus. `O(1)` *addressing* versus `O(N)` search is the ledger's whole economy |
| AYE8: a fixed binary payload with magic, version, fields | the `-absorbed` header: a stable envelope so any consumer can read the spine without the whole file |
| Goertzel: compute only the bins you need | compress to the spine — the same discipline as these rows, `O(N·K)` instead of `O(N log N)` |
| `the-stack-absorbed.md` drew Phoenix/Marine/MEM\|8 as theory | **this row is that theory, running.** The stack was a drawing; here it is a `cargo build` with 100%-correlated blocks |
| `hyperbionic-reading-absorbed.md`'s Πaudio = Πmem = Πtext | the Marine algorithm is the shared primitive: one `O(1)` salience front end, "modality-agnostic", reading a DDoS spike and a high note as the same event shape |
| "We're not replacing the OS. We're liquefying it." | the constellation's stance in six words — layer over the substrate, never fight it |

## the honest note

**The prose is larger than the code, on purpose.** Two narrative pieces
ship in the repo root tell a story in which FFT is *replaced* by a time
domain model and "the AI restores the missing context by calculating the
necessary values from the surrounding wave geometry." The implementation
does not do that. It uses **Goertzel** for its mel frames — which is a
Fourier technique (a per-bin DFT), chosen for *cost*, not a rejection of
the frequency domain ("Goertzel over FFT", explicitly). So the repo holds
two registers: a vision in the future tense, and a fairly modest v1 that
transcribes audio and clusters jitter. The row keeps both; it does not
let the vision's tense leak into the results. The `O(1)`/resonance
claims belong to **Phoenix v2, marked "planned"** — today's Phoenix is
vector features and cosine similarity, which the README itself calls
"training wheels."

Two smaller hedges, kept rather than smoothed: the "97-100% accuracy" is
a headline with no benchmark harness in the docs; and the performance
figures come from two different runs (18 s audio vs. 3 s on an M1, with
"simplified mode" LM forward) that should not be quoted as one number.
The security allowlist and the "sub-millisecond retrieval" are the
authors' claims; neither is independently measured here.

The other honest note is the wiring the request named: this lands in
`8b-is` because the repo is `8b-is/liquid-rust`, authored by the same
hand that signs `standardgalactic` — and it is *about the surfaces* this
constellation already runs (`pocoo.vaked.dev`, `art.vaked.dev`, the
`mem8` daemon at `~/.mem8/`). One repo, three names on the door.

*implementation · `8b-is/liquid-rust` · LFM2.5-Audio in pure Rust ·
Phoenix Protocol · Marine Algorithm · MEM\|8 / AYE8 · diarization by
physics · the constellation · fine touch from within · vaked.dev ·
8b-is · standardgalactic · 2026-10-14*
