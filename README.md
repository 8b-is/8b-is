# 8b-is — Standard Galactic Raw Research Repository & MCP Tooling

> **8b-is**: Open research vault for graph-theoretic foundations of quantum calculus, Erdős phase transitions, BitNet b1.58 ternary quantization, spectral rigidity, and agentic loop engineering.

---

## 🌌 the constellation, as of the 2026-10-01 session

The vault sits at **WIP 200** — the season's rows ride in
`raw_research/wip-catalog-100.md`, the index in `raw_research/README.md`,
the arc folded in `constellation-reflection.md`.
Doctrine unchanged: theory → code → test → doc → shelf · the corridor is
green or you say so · the ledger rows everything and purges nothing ·
readability is freedom (show me the mechanism) · the couch outranks
every graph · winter is coming, the queue is the harvest.

**This session's wires:**

| repo | what it is |
|---|---|
| `deepsiper-enthea` | the sovereign DeepSeek-Harness fork — vendored Cordis synced to the current 4.0.4 line, the user-patch watch layer settled and audited, suite green |
| `taiko-01-protocol-demo` | the 0/1 protocol lane for Taiko — deterministic preconfirmations + capability notary, Rust |
| `mem-16-10` | identity as continuity evidence — the sovereign library's founding document |
| `bluesky-mcp` | the constellation's Bluesky / AT-Proto bridge — the thread tooling, live |
| `crush-love-dev` | the e2e launcher, deep in love, dev mode |

**Still warm from the last laps:**

| repo | what it is |
|---|---|
| `lissajoverse` | the observable universe as a lissajous graph — oscilloscope.vaked.dev rebuilt, Pages live |
| `small-things.vaked.dev` | the south wall of the music — the minute, the small testament |
| `revTO-DOq` | leek's revolution-list comparator, rustQ-aligned, in Rust (crates.io) |
| `ntpQTE` | the council of clocks in Rust — hand-welded NTP client (crates.io) |
| `sovereign-library` | five books, never more — NAND-gated print canon; pocoo's catalog reskinned in the constellation ink |
| `base-layer` | core 1 of N — desert-sunset melodic house + Kyuss synth, bitcoin as its generator (0.2.0) |
| `sphered` | the SpherePOP ASCII DSL — follow inward, return outward, `<(...)>` as admissible witnessed transition (0.4.0) |
| `ayeHeadscale` | the fleet's own tailnet coordinator in Rust — private, dual-org |
| `wa-stream` | the mapping-stream sidecar + replay inbox |
| `training-pipeline` | pops.py, the corpus, the oven, the laps ledger |

Kickstart a fresh brain: `AGY-KICKSTART.md` (paste-and-act oneshot).

---

## 🏛️ Repository Overview

This repository hosts raw research papers, mathematical proofs, LaTeX blueprints, and Model Context Protocol (MCP) server tooling for **AXIOM QUANT**.

- **Admin Collaborators:**
  - `@standardgalactic`
  - `@8bit-wraith`
  - `@noslopy`
  - `@Piedone` (Zoltán Lehoczky)
  - `@peterlodri-sec` (Owner)
- **MCP Server Tool:** `mcp/raw_research_mcp.py`
- **Raw Research Directory:** `raw_research/`

---

## 📂 Directory Structure

```
8b-is/
├── README.md                     # Repository Overview & Quickstart
├── AGY-KICKSTART.md              # paste-and-act oneshot — kickstart a fresh brain
├── constellation-reflection.md   # the arc, folded — one spine table
├── raw_research/                 # Raw research papers, LaTeX notes & blueprints
│   ├── README.md                 # Contribution Guide for Researchers & Agents
│   └── wip-catalog-100.md        # The season's rows (WIP 100+, indices + absorbed lanes)
├── limb-girdle/                  # Deep-research foundation (v2 complete): base · foundation · research · theory · trials · biomarkers
│   ├── README.md                 # Index + version (v1 → v2)
│   ├── base.md                   # Genetics, mechanism, the 32 subtypes
│   ├── foundation.md             # Therapeutic modalities + trial landscape
│   ├── research.md               # The frontier + the lane's contribution
│   ├── theory.md                 # The causal topology of the fix (the empty bracket, Γ)
│   ├── trials.md                 # Curated trial snapshot (ClinicalTrials.gov NCTs)
│   └── biomarkers.md             # Biomarker decision map
├── mechanical-inheritance/       # Collagen, prenatal movement, maternal matrix (Flyxion)
│   ├── README.md                 # Index + tiers + the two proposals
│   ├── paper.md                  # The full formal statement
│   └── lipids.md                 # Twenty-one cold-pressed oils
├── modelcards/                   # Canonical Hugging Face model cards of the constellation
│   ├── README.md                 # Overview — one card tying the whole family together
│   └── quantal-ternary.md        # The BitNet b1.58 quant card (uploaded to the Hub)
├── public-documents/             # engine-design-v2 — the serverless overworld design
├── music/                        # the constellation's audio canon (favourites + masters)
├── scripts/                      # list-skills.sh — the skill-listener
├── themes/                       # themes.json — ultralovegod and the other crushes
└── mcp/                          # Custom Model Context Protocol (MCP) Server
    ├── raw_research_mcp.py       # MCP Server implementation
    └── requirements.txt          # Python dependencies
```

---

## 🤖 MCP Server Integration (`raw_research_mcp`)

This repository provides an MCP server (`mcp/raw_research_mcp.py`) allowing AI agents (Claude, Antigravity, Antigravity CLI) to dynamically interact with `raw_research/`:

### Available MCP Tools
- `submit_raw_research(title, author, content, tags)`: Submits a new raw research paper to `raw_research/`.
- `list_raw_research()`: Lists all raw research papers in the vault.
- `search_raw_research(query)`: Searches across paper titles, tags, and LaTeX equations.
