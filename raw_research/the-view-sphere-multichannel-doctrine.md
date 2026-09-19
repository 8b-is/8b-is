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