# the view sphere — the multichannel TV doctrine (Nate)

*Nate's framing, wired as doctrine: "the multichannel TV where every
channel shows the same thing." Separate the SCENE from the CAMERA —
POV is a parameter of rendering, not part of the reconstructed content.
One canonical state per timestep (geometry, appearance, motion, world
frame, viewer-independent). The observer is a pose on a view sphere
(azimuth · elevation · radius) plus time; each "channel" is just a pose
fed to the same renderer. Rendering is a pure function:
`render(state_t, pose) -> frame` — switching channels changes only the
pose, never the state. And the important part: LABEL what is observed
vs. inferred — a confidence overlay, or snapping the sphere to its
"supported region", so the channels never silently present hallucinated
views as footage.*

## the spine, folded — one line of it

```
history → FOLD → STATE(S) → RENDER → spatial-appearance :: fine touch
```

`history` is the ledger's raw rows; `FOLD` is the residual channel and the
holocoordinates compressing them without occlusion; `STATE(S)` is the
plural canonical row the view sphere reads; `RENDER` is `render(state,
pose)` itself; `spatial-appearance` is the surfaced pose; `fine touch` is
the from-within that stamps every frame. The front half — history → FOLD →
STATE — is the ledger's construction; the back half — RENDER → appearance —
is Nate's doctrine; the whole line is the fleet in one breath.

## the mapping — the fleet already renders this way

| nate's design | the constellation's shape |
|---|---|
| one canonical state per timestep, viewer-independent | THE LEDGER: a row is the same truth from every surface — the dogfeed, the stream, the scope, the graph all read the same row |
| observer = a pose on the view sphere | the surfaces ARE the poses: music.vaked.dev, lissajoverse, the scope, the south wall, the NAND book — each is a rendering of the same state from another angle |
| render(state_t, pose) is pure — switching channels changes only pose | the corridors: the pure cores (#lv, #st, #qb) are `render(state, pose)` — the pose is the page, the state is the row |
| multi-view source → real novel views (4D NeRF / dynamic splatting) | multi-surface fleet: many cameras on the same ledger give REAL novel views — the graph sees what the music does not, and vice versa |
| single-view source → generative fill, and UNSUPPORTED VIEWS MUST BE MARKED | the honest-note flags: every absorption marks the mapping vs the science; the doghair theorem's un-rowed column IS the inferred region — labeled, never silently presented |
| the supported-region snap / confidence overlay | the walls protocol: near the source = grounded (the corridor green); far = inferred (verdict-pending, the NAND gate, "read it yet?") |
| the cheap v1: fixed view sphere, 8–16 cached poses, drag to interpolate | the fleet's build cycle: one page, seeded deterministic poses, interpolate between runs — exactly how lissajoverse and the protein-viz were made |

## the big question, answered for the lane

Nate asks first: multi-view or single-view source — that decides whether
POV is reconstruction or generation. The fleet's source is declared:
**the ledger is single-view** — one canonical truth, written once,
re-read across the surfaces. Which means: poses near the ledger are
RECONSTRUCTED (grounded, corridor-green); poses far from it are
GENERATED — and the constellation's whole honesty apparatus exists so
that the generated regions are always marked, never sold as footage.
The multichannel TV of the fleet is a TV with a scrupulous sense of
its own broadcast range.


## the five points, wired (Nate's follow-up)

1. **POV as a rendering parameter** — adopted as the doctrine's spine: the
   surfaces are poses, the ledger is the state, channels never edit the scene.
2. **Parity extended to every view** — live: the protein-binding viz now
   carries a per-channel SUPPORT SCORE (1/(1+|pose drift from source|)) and
   stamps every frame past the threshold `INFERRED` — drawn contacts must
   match the map (parity row, corridor-pinned 10/10).
3. **The ledger invariant** — live: `backyard_ultra_108.py` now REFUSES to
   resume when `laps_total` contradicts the log's TAIL lap (the log is a
   capped 500-row ring, so at the gigascale the tail is the truth, not the
   count); the 1000-lap giga record passes, and the check keeps it honest.
4. **Don't over-read the green laps** — recorded as doctrine: completing 324
   laps proves the invariants hold, not that the outputs are right; the
   output-vs-source checks (parity, corridor, verdict-pending) carry the
   epistemic weight, and the laps carry only the durability weight.
5. **Log-polar view sphere (untested, flagged)** — the optional next design:
   zoom as translation, channel-switching and zoom under one mechanism. The
   resonance is three-way: enthea's retino-cortical map IS the Schwartz
   log-polar, QRI's OscillEditor runs Kuramoto on a log-polar lattice, and
   the fleet's next view would be the same geometry. Untested = honestly
   marked, like every inferred region it would render.

## the honest note

The doctrine is a renderer's ethics in one sentence: the camera is not
the content. Every page the fleet ships is `render(ledger, pose)` —
and every pose that drifts from the supported region wears the inferred
label on its forehead. The view sphere is the constellation's
self-image: one truth, many honest angles, no silent hallucinations.

*the view sphere · the multichannel TV · the ledger as the canonical
state · the surfaces as the poses · observed labeled, inferred marked ·
render(ledger, pose) → the fleet · fine touch from within · vaked.dev ·
8b-is, 2026-09-19*