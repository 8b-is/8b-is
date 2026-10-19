# Topic 23 — Taiko 0/1 Preconfirmation Protocol, Genesis Seal, Zig Kernels & Ring-Fuse

> **Pillar IV** · **KOMPRES Invariant:** `Gate(tx) ∈ {Sign, Refuse} ∧ EmptyFrame => Resync(±3)`
> **Standard Galactic & 8b-is Rooms:** `8b-is/taiko-01-protocol-demo`
> **Shard Budget:** `3` raw research notes · `12,050` bytes (`~3,011` source tokens compressed losslessly via metadata sharding)

---

## 1. Load-Bearing Thesis & Atomic Invariant

```text
TOPIC_23 := Gate(tx) ∈ {Sign, Refuse} ∧ EmptyFrame => Resync(±3)
```

## 2. Absorbed `8b-is/raw_research/` Shards (Lossless Links)

### [The Genesis Seal and the Cross-Language Kernel Lane](../../raw_research/genesis-seal-and-the-zig-lane.md) (`genesis-seal-and-the-zig-lane.md` · `2,858 B`)
- **Abstract Shard:** Recorded into the 8b-is vault because the constellation's installation story grew a spine: the installer is multi-part, the installer's truth is a public gist, and the kernels now speak a second language. The engine runs without you — and it verifies itself out-of-band.
- **Section Spine:** `the genesis seal` · `the cross-language kernel lane (Zig through the C ABI)` · `the engine transfer`

### [the message plane — ring, queue, and the fuse](../../raw_research/the-message-plane-ring-and-fuse.md) (`the-message-plane-ring-and-fuse.md` · `3,406 B`)
- **Abstract Shard:** Péter's architecture note, prompted by this session's lived symptom: empty user messages — transport frames that arrive with no payload. The proposal: treat an empty frame as a RESYNC EVENT; fuse the last ≈3 messages and refeed them into thinking; and run tool calls + responses
- **Section Spine:** `the mapping — the machinery that already IS the ring` · `the doctrine, stated` · `the honest note`

### [EOS-CLA v1.0 — Eternal Open Substrate & Contributor Mutual Trust Agreement](../../raw_research/eos-cla-v1.md) (`eos-cla-v1.md` · `5,786 B`)
- **Abstract Shard:** Drafted in the game-studio thread (8b.es + Péter József Lódri), received 2026-09-06. The legal cornerstone of the studio: eternal open source, mutual trust, AI-native contribution. The constellation's admissibility contract in legalese — replayable ⇒ admissible; survivable ⇒ low load ∧
- **Section Spine:** `Preamble` · `1. Definitions` · `2. Eternal Open Source Grant & Ownership` · `3. AI Agent & Swarm Symbiosis (AIB Integration)` · `4. Absolute Disclaimer of Liability ("The Crash-and-Burn Clause")` · `5. Representations & Mutual Trust`

---

*[← Back to 42-Topic KOMPRES-ULTRA-OMNI Atlas](../README.md) · `[Topic: 23/42 | Pillar: IV | Shards: 3 | Lossless: True]`*
