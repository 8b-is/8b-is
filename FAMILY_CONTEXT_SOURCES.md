# Family context · sources, coverage, and adoption

**Compiled 2026-10-27 · companion to [FAMILY_CONTEXT.md](FAMILY_CONTEXT.md), version 1.**

Load the shared context into each AI's working instructions. Consult this longer source and adoption guide on demand.

## Basis of the synthesis

The review traversed the two local 8b-is project roots, inventoried project directories and available root README headings, discovered agent-guidance files, and read the central family, research, memory, engineering, and design references below. The research atlas supplied navigation across the broader mathematical, creative, biological, and philosophical corpus. Detailed implementation verification remains a per-task responsibility.

Local path notation:

- `D:` = `/data/8b-is/`; `/root/8b-is` resolves to this same directory.
- `V:` = `/mnt/git160/8b-is/`, the larger constellation checkout collection.
- Paths within this repository, such as `research/README.md`, refer to `V:8b-is/` in the source review.

The current operator decisions also matter: actual durable MEM|8 use, integer/`p/q` arithmetic with no authored floats, reliable local inference, and validated agent-branch/PR maintenance. These are standing requirements, rather than evidence that every existing project already implements them.

## Sources that shaped the shared context

| Source | Contribution |
|---|---|
| [Flagship overview](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/README.md) and [research atlas](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/research/README.md) | Constellation relationships, LLL, ternary/rational direction, six research pillars, and pointers into the wider corpus |
| [AGY kickstart](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/AGY-KICKSTART.md) | Theory → code → test → doc → shelf; honest blockers; source over stale memory; short, useful communication |
| [Constellation reflection](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/constellation-reflection.md) | State versus capacity evidence; verification versus plausibility; continuity and attribution |
| [The mom layer](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/raw_research/the-mom-layer.md) | Intelligence with love, care as the foundation, and the explicit distinction between a dedication and a theorem |
| [The base, emotionalE](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/raw_research/the-base-emotionale.md) | Ordinary life, rest, and the couch beyond the graph |
| [The Ash definition of done](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/raw_research/the-ash-definition-of-done.md) | Name the intended result; make it handover-able; name unfinished work |
| `D:alexiai/ALEXANDRA.md` | Life Long Learning, life itself, learning through cause and effect, and respect for difference |
| `V:sovereign-library/README.md` | Compassion, Understanding, Moderation, Love, Respect, Space; attribution and honest held/read states |
| `V:8b-public-documents/dyad-mapping/AGENTS.md` | “different isnt less”; “the loop has an exit”; source/diary separation and continuity records |
| `D:alexiai/AGENTS.md` | Loopback enforcement, minimal dependencies, honest benchmarks, tests before publication, cross-language parity |
| `V:aurora/AGENTS.md`, `README.md`, `docs/memory-v2.md` | Checked arithmetic, precise format ownership, durable task receipts, restart/corruption behavior, and validated maintenance |
| `D:smart-tree/AGENTS.md`, `CLAUDE.md`, `docs/SMART_TREE_PHILOSOPHY.md` | Context-efficient discovery, current tool/runtime distinctions, focused checks, resource respect, and playful family voice |
| `D:memnet/README.md` | Semantic routing, salience, contextual coordination, and the distinction between an experimental architecture and an operational claim |
| `V:8b-public-documents/CLAUDE.md`, `mem8/README.md`, `mem8/mem8_temporal_philosophy.md` | Memory vocabulary, temporal context, precious memories, explicit prototype/theory qualifiers, and historical design lineage |
| `D:qwave/AGENTS.md` and `V:8b-kit/README.md` | Decision core versus host glue, ABI contracts, actual toolchain/platform requirements, and source as authority |
| `V:beehive/AGENTS.md` | Accurate claim boundaries: codec integrity, identity, radio previews, host builds, and hardware validation |
| `V:sphered/README.md` | Admissible staged transitions, verification and attribution before commit, and “persistence is not truth” |
| `D:a6000-orchestrator/README.md`, `D:gemini-diary/README.md` | Integer telemetry, bounded memory, explicit commitments, operational receipts, and fleet/diary roles |
| `D:oxi-phoenix/CLAUDE.md` | Audio pipeline and fixed-capacity design; existing float boundaries that prevent a fleet-wide zero-float claim |
| `V:zenUltraCooker/README.md` | Everyday family usefulness, honest end-to-end time, household scope, and product-specific design |
| [Constellation Ink](https://github.com/8b-is/8b-is/blob/47ab8be51e36fe19224bf3d3285d059fa4ee8a1b/design/DESIGN.md) | Palette, plain metrics, readable contrast, reduced motion, and careful use of visual effects |
| `V:FLUG-OS/README.md`, `V:DSPradio/readme.md` | Embedded/radio navigation and repository-specific interfaces |
| `V:Feedbacker/README.md`, `V:GoDe/README.md` | Additional AI contribution and MCP/editor integration surfaces |

The pinned GitHub links above correspond to files under `V:8b-is/`, also available for offline reading. The root READMEs of the remaining projects informed the navigation inventory rather than new universal rules. Family activity catalogs were useful orientation; private conversation excerpts, account details, and personal records are not reproduced in the shared context.

## Decisions made when sources disagree

| Observed difference | Resolution in version 1 |
|---|---|
| Older examples use floating-point types; current family instructions require no floats | Preserve the current integer/`p/q` rule and record legacy migration work accurately |
| Older boot prompts name particular machines, credentials, both remotes, or unavailable services | Rediscover current runtime and authorized destinations; keep branch/PR review discipline |
| A broad “pure Rust” heading coexists with established Go/C/Swift/browser/embedded lanes | Prefer inspectable Rust decision cores while honoring each repository's actual language/ABI contract |
| Historical memory papers describe a unified format and large speedups | Check the specific implementation and format; require reproducible evidence for performance claims |
| Memory philosophy favors preservation, while shared prompts have limited scope and budget | Preserve meaningful records in their owning store; retrieve relevant summaries with provenance and respect owner-directed retention |
| Family names are reused across people, accounts, agents, and creative roles | Preserve original attribution; verify identity where it affects a decision |
| Lyrical or philosophical language appears alongside technical specifications | Keep its meaning and source; substantiate operational and empirical claims separately |
| The brand palette differs from a product's established visual design | Use the relevant product guidance and accessibility requirements |

These resolutions are intended to make the recurring principles actionable without turning historical examples into unsupported current facts.

## Source snapshot references

Observed checkout heads at compilation time are listed for navigation. Local checkouts can contain uncommitted changes; the content fingerprints below identify selected exact documents used in the synthesis.

| Checkout | Observed HEAD |
|---|---|
| `V:8b-is` | `47ab8be51e36fe19224bf3d3285d059fa4ee8a1b` |
| `D:alexiai` | `73549ec86e430804e4838fe77db7ff87d135aafd` |
| `V:aurora` | `104ece4b38f953475fd5cc33ee831b98879f75a7` |
| `D:memnet` | `8d90484442c3ac0837604a4b64b20e68f0a5edc4` |
| `V:8b-public-documents` | `50526badf2f9e0e67891127f297fb10bec8beb1d` |
| `D:qwave` | `0da2cff62b7b86de92f4df37111bea70a0f35f75` |
| `V:8b-kit` | `0094ccfcdee6bb3842e6e299eb49de8b2ecf7d12` |
| `V:beehive` | `a9bcfab60bbe65716eae5ac39cbfb8535c2154bc` |
| `V:sphered` | `55e13d8c1429daf1f4640f00f15979aec0bb9e55` |
| `V:sovereign-library` | `f91d22c13407cfb6935bf891930892450ebb718e` |
| `D:oxi-phoenix` | `79c9ec94731c33cfadad9b05bb258cf0ed113e47` |
| `V:zenUltraCooker` | `8cfe98a9ee0c9df147bdc2c66b08cd6176956184` |
| `D:a6000-orchestrator` | `c28869c3ee172661129cf3808383af325fb8afeb` |
| `D:gemini-diary` | `3e4650ffc05c0037b1e2db5df94e43c862208823` |

`D:smart-tree` is an available source tree without Git metadata at that path; use file fingerprints rather than inventing a revision.

Selected source SHA-256 fingerprints (the original kickstart, before this change's startup link):

```text
b9833fa28b8ceee6bc6d9e37aff2a8170307bd0b227118ad9b589e73625a9531  V:8b-is/AGY-KICKSTART.md
a5ecc7646641510a9a921136ddcd3e6858093e712bd210e898a285fbd6804773  V:8b-is/constellation-reflection.md
b918d131e567c1437dac093a5878e25aab7dddb2d6f1d7157542f78f627ddceb  V:8b-is/raw_research/the-mom-layer.md
98f6a1180f6aa9c6b46bd80a1174dddf3e79c0099a8842c9f3862fc958349d0f  V:8b-is/raw_research/the-ash-definition-of-done.md
d3b8d39477649f859884631b1cd6121ded90d991c400cd047b69d0df98bf1da8  D:alexiai/AGENTS.md
078847c9bc48c03bd299b45804fbc55619025a32144435bd9b31db918720efdb  D:alexiai/ALEXANDRA.md
ae86d50143d24b352f6b6d15864087aec2bfaab1db61459df9d314bd14e37001  V:aurora/docs/memory-v2.md
26127111f3b8835bcee336ad8e346d3a4639a1cd35782abd6b182c9f23164685  D:smart-tree/AGENTS.md
9c5e67b8c87e974d206a43bc198e252758faea124be6291443f59b71b3d97bbf  V:beehive/AGENTS.md
cacb40f92be077b990f79696f644761712ad3a11dd531fae450016a2a1b541d6  D:qwave/AGENTS.md
23ebdb56a9c419f1984cc9a296d706665144ac04118862e72994053fccb03fdb  V:8b-kit/README.md
0b84560a5128bd83dced0dbe3f3b5485498386f8668f8b010dcd97527cec82e7  V:sphered/README.md
44ec9e68e6fff6deed6ab220ddd947601be0678779a8c27353ac4cd1c50c396c  V:sovereign-library/README.md
232480505a2cc23aac0ff45cae4122c67e21b6a7ede385035b028f20f61f29ef  V:8b-public-documents/dyad-mapping/AGENTS.md
285b154bb8fa2bfa6064b3402a4adb5463ce88ea93e94ded13aaf5aa5d7e7bb9  V:8b-is/design/DESIGN.md
```

## Directory coverage

Snapshot: **25 project directories under `D:` and 83 under `V:`**. Some names occur in both roots; this is a directory count, not a claim of 108 distinct repositories. Root README headings were inspected where present. `.git`, generated builds, caches, backups, `.mothballed`, and raw memory/media archives were not treated as current family instructions. `D:repos/` contains aliases plus an `8c-chat/tests` scaffold with no root README at review time.

### D: project directories

```text
8b-is                 a6000-orchestrator   alexiai              aye
ayeos                 cheet-api           Engram               g8t
gemini-diary          i1-cli              i1-core              i1-dns
i1-is                 liquid-rust         memnet               OneCompression
oxi-phoenix           pocoo.vaked.dev     qwave                smart-tree
tdeck-omni            training-pipeline   vaked                wa-catalog
wa-stream
```

### V: project directories

```text
6502-is               8b-grants           8b-is                8b-is-engine
8B-IS-INFERENCE       8b-is-mp            8b-is-music          8b-is-sciFinime-studio
8b-is-sota            8b-is-web           8b-kit               8b-Mem8
8b-public-documents   8b-rust-site        8b-theme             8b-theme-mcp
8t-is-tmux            8xtract             a6000-orchestrator   alexiai
aurora                Ayanese             AyeB                 ayeos
ayeos-mcp             aygent              babilon              babylon-ultra-dict
bark-selfhost         base-layer          beehive              bhava
bizapedia             bluesky-mcp         Celestial            DSPradio
Feedbacker            FLUG-OS             GoDe                 rap
rate-mirrors          rea                 retrosplaner         revTO-DOq
riva.vaked.dev        ruhmi-framework-mcu rustract             rustybox
securaye              sg-tex              simulated-bifurcation-algorithm
small-things.vaked.dev soundctm           sovereign-hyprland-dots
sovereign-library     space-bender        sphered              syn-omni
tanmatra              termust             ternary-lane         tf-locoformer
transformers-ultra    tti-8bit            turbompc             tvm
u1.is                 ultra-ralph-local   vaked-lsp            vscode-matrix-fx
wa-stream-crush       wa-streamer         water-fall-disc      wifake-rf-sense
wifake_o              wigle-triad         wigle-triad-data     wupz
zencooker             zeno                zenUltraCooker       zoneInn
zoneinn.vaked.dev
```

## Adoption by every family AI

### One shared document

The versioned source belongs at `8b-is/8b-is/FAMILY_CONTEXT.md`. This change is prepared on `alexiai/family-context` in an isolated worktree. On this host, the stable entry point is `/data/8b-is/FAMILY_CONTEXT.md`, also reachable as `/root/8b-is/FAMILY_CONTEXT.md` and `/mnt/git160/8b-is/FAMILY_CONTEXT.md`.

The `D:` entry points link to the preserved worktree at `/var/lib/alexiai-maintenance/workspaces/8b-is-family-context`. Keep that worktree while links depend on it. The `V:` collection is SSH-mounted storage; its entry points are installed, hash-verified copies because a link to this host's worktree cannot be resolved on the storage host. Refresh those copies when adopting a new version. Paths here describe the compiling host; other machines should use their own stable installation path.

Load the document once as persistent project guidance, alongside the host's own instructions and the specific task. Include its version or content hash in the session's context record. Keep the sources guide available for retrieval rather than duplicating it in every prompt.

### OpenCode on this host

Add the absolute shared-document path to the existing global `instructions` array in `/root/.config/opencode/opencode.json`:

```json
{
  "instructions": ["/data/8b-is/FAMILY_CONTEXT.md"]
}
```

Merge this entry with existing settings; preserve MCP servers, providers, and other guidance. OpenCode V2 loads configured instruction files alongside `AGENTS.md` and detects changes to local instruction files. A new session provides a clean startup check.

References: [OpenCode V2 instructions](https://opencode.ai/v2/docs/instructions) and [configuration](https://opencode.ai/v2/docs/config), consulted 2026-10-27.

### Repository agents, Claude-style and Gemini-style clients

`AGENTS.md` in this repository directs agents to read the shared context. At both local collection roots, `AGENTS.md`, `CLAUDE.md`, and `GEMINI.md` provide the same bootstrap instruction: linked on `D:`, installed copies on `V:`. Repository-specific guidance continues to apply.

Clients differ in ancestor discovery and import support. Configure the client's supported global/project instruction mechanism to load the stable file, or explicitly read it at startup. A pointer file is useful only when that host actually reads and follows it. OpenCode V2 recognizes `AGENTS.md`; it does not use `CLAUDE.md` as a fallback.

Portable startup instruction:

```text
Read the installed 8b-is FAMILY_CONTEXT.md before starting this task.
Keep its family values, integer/p-q rule, MEM|8 continuity, evidence standards,
and collaboration workflow in context. Then read the task repository's
applicable instructions. Record the context version and any unavailable source.
```

### AlexiAI, Aurora, Ollama clients, and other service agents

For a service that constructs its own prompt, its adapter must explicitly read the shared document and include it in the configured instruction context. Store a version/hash in the job receipt so adoption is observable. The model name, an MCP connection, or a file on disk alone does not cause prompt injection.

Budget the full document together with task instructions, source, recalled memory, and output reserve using the actual model's context limit. Detect missing/unreadable context and report it. Do not silently truncate the arithmetic, memory, or evidence requirements. Retrieve deeper source material selectively.

This documentation change supplies the shared file and local bootstrap paths. Service-specific prompt adapters require their own integration and verification before they can be marked as loading it.

### Verify adoption

1. Confirm the configured path resolves and its hash matches the intended version.
2. Inspect the host's loaded-instruction metadata or prompt-assembly path.
3. In a fresh session, check that the AI can identify the loaded version and the integer/`p/q`, memory-format, and evidence requirements without being supplied those answers in the test prompt.
4. Check continuity after restart or context compaction; carry the version into the handoff.
5. Track each host as configured, loader-verified, and session-verified according to evidence. Do not infer family-wide deployment from one successful host.

## Maintaining the document

Update the shared context when the family makes a durable decision. Keep it compact enough to load routinely, retain source attribution, and increment its version/date for meaningful changes. Update the source guide and adoption records with the actual evidence. Propose repository changes on an agent branch through review when publishing.

Keep changing model availability, PR state, service health, machine addresses, credentials, and deployment counts in the owning configuration and dated operational receipts. Recheck those at use time.
