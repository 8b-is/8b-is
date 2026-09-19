# 8b-is — Standard Galactic Raw Research Repository & MCP Tooling

> **8b-is**: Open research vault for graph-theoretic foundations of quantum calculus, Erdős phase transitions, BitNet b1.58 ternary quantization, spectral rigidity, and agentic loop engineering.

---

# 8b-is — Standard Galactic Raw Research Repository & MCP Tooling

> **8b-is**: Open research vault for graph-theoretic foundations of quantum calculus, Erdős phase transitions, BitNet b1.58 ternary quantization, spectral rigidity, and agentic loop engineering.

---

## 🌌 the constellation, as of the 2026-09-18 session

The vault sits at **WIP 161** — the season's rows ride in
`raw_research/wip-catalog-100.md`, the index in `raw_research/README.md`.
Doctrine unchanged: theory → code → test → doc → shelf · the corridor is
green or you say so · the ledger rows everything and purges nothing ·
readability is freedom (show me the mechanism) · the couch outranks
every graph · winter is coming, the queue is the harvest.

**The new session repos, wired into the org:**

| repo (github.com/8b-is) | what it is |
|---|---|
| `lissajoverse` | the observable universe as a lissajous graph — oscilloscope.vaked.dev rebuilt, Pages live |
| `small-things.vaked.dev` | the south wall of the music — the minute, the small testament |
| `revTO-DOq` | leek's revolution-list comparator, rustQ-aligned, in Rust (crates.io) |
| `ntpQTE` | the council of clocks in Rust — hand-welded NTP client (crates.io) |
| `sovereign-library` | five books, never more — NAND-gated print canon |
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
├── raw_research/                 # Raw research papers, LaTeX notes & blueprints
│   ├── README.md                 # Contribution Guide for Researchers & Agents
│   ├── paper_template.md         # Markdown / LaTeX research template
│   └── 01-sample-blueprint.md   # Sample raw research document
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
