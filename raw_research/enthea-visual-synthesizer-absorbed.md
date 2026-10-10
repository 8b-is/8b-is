# ENTHEA, absorbed — the visual synthesizer that shows its equations

*viz.vaked.dev is not a drug simulator with pretty shaders. It is a
catalogue of ~30 phenomena, each one named, sourced, and rendered from
the mathematics that produces it — with a flag on every entry that is an
evocation rather than an exact solution. The operator pointed at it on
the morning after the 2.5-hour session. What is absorbed here is less
the art than the discipline: a shader library that keeps the same line
this vault keeps between a rendering and a derivation.*

## the object

An altered-states visual synthesizer running in the browser (WebGL2).
A neural-field / reaction–diffusion substrate feeds a compose pass; dose
drives cortical gain and coupling so geometry appears and sharpens as
you come up. Colour is computed in **OKLab**, not sRGB, so the ramps are
perceptually even. Source can be mic, file, or captured machine audio;
the `♒ scope` rite overlays the real waveform on any mode.

## the modes, as declared

Each mode carries its math and its sources. Representative entries:

| mode | the mathematics | source |
|---|---|---|
| form constants | cortical plane waves inverse-mapped through the retina→V1 **complex logarithm** → tunnels, spirals, lattices | Klüver (1928/1966); Bressloff–Cowan–Golubitsky–Thomas–Wiener (2001) |
| neural field | **Wilson–Cowan / Amari** field on a cortical grid, Mexican-hat coupling, Turing bifurcation with non-zero critical wavenumber | Ermentrout & Cowan (1979) |
| turing flux | **Gray–Scott** reaction–diffusion integrated live | Turing (1952); Pearson (1993) |
| sacred geometry | dihedral symmetry Dₙ folded over real **phyllotaxis**, golden angle 137.507° | Vogel (1979); Douady & Couder (1992) |
| dragonscales | the ocellated lizard's skin as a **living cellular automaton** — discrete Turing on a hex lattice | Manukyan et al., *Nature* (2017) |
| weierstrass wells | **domain colouring** of ℘(z), the double pole winding the hue by −2 per cell | Weierstrass; Wegert (2012); DLMF §23 |
| blaschke rosette | degree-n **finite Blaschke product**, coloured by arg B′ — n zeros, n−1 critical points | Garcia–Mashreghi–Ross (2018); Heins (1941) |
| indra's necklace | **Schottky/Kleinian** limit set by inverse-orbit escape time | Mumford–Series–Wright, *Indra's Pearls* (2002) |
| arnold tongues | **sine circle map** parameter plane; at K=1 the locked intervals fill the devil's staircase | Arnol'd (1961); Jensen–Bak–Bohr (1984) |
| pentagrid loom | **de Bruijn's pentagrid**, whose dual is exactly the Penrose tiling | de Bruijn (1981) |
| vortex condensate | **Abrikosov** vortex lattice — quantized vortices in a superfluid order parameter | Abrikosov (1957); Tkachenko (1966) |
| particle flow | 50k GPU particles advected through the **ABC flow**, an exact divergence-free solution | Arnold (1965); Dombre et al. (1986) |
| quasicrystal | N plane waves at evenly-spaced angles → aperiodic N-fold interference | de Bruijn (1981); Shechtman et al. (1984) |

Plus cymatics (Chladni 1787 nodal lines), Voronoi/Worley (1996),
phasor noise (Tricard et al. 2019), atomic orbitals, the modular/Farey
tessellation, sine-Gordon breather lattices, Gaussian primes, Mandelbox
raymarching, and the entoptic modes (visual snow, Scheerer's blue-field,
1924).

## the honesty flags — the part that belongs in this vault

Every mode that is an *evocation* rather than an *exact* construction
says so, in the interface:

- hyperbolic geometry — flagged **unverified theoretical hypothesis
  (blog-level sources), included as a frontier idea — not established
  science** (the QRI negative-curvature claim)
- phasor noise — flagged a **real-time approximation (kernel sum per
  pixel)**, not the full spectral construction
- Indra's necklace — flagged the **stable four-circle construction**, not
  the exact parabolic cusp; the distance estimate is approximate
- Weierstrass — flagged the **7×7 (48-term) Eisenstein truncation** and
  that the τ-morph is a hand-chosen visual path, not a rigorous modular
  flow
- Arnold tongues — flagged that W is estimated from ~30–48 iterations
- wave crystal, defect gas, vortex condensate, hyperspace, fractal —
  each flagged as evocations of a PDE, a chemistry, or a
  phenomenological report rather than solved systems

## the mapping

| the synthesizer's move | the constellation's shape |
|---|---|
| every mode declares `math` + `refs` + `flag` | the `-absorbed` row's format exactly: the spine, the exact statement, the honest note |
| a `flag` where the render is an evocation, not a derivation | `σ_d` against `ω`: the annotation states the component's real evidential support, so it cannot overclaim |
| dose drives cortical gain, so geometry *emerges* rather than is drawn | the lane's preference for emergent over scripted — the pattern is a solution, not an asset |
| OKLab instead of sRGB | perceptual honesty: the ramp the eye sees is the ramp the math names |
| the 𝒾 card cites the papers next to the shader | readability as freedom, in a fragment shader — show me the mechanism |
| a `SAFE_GENERIC` line on every substance entry | the register's guardrail: phenomenology, explicitly not dosing or medical advice |

## the honest note

The mathematics, sources and flags above are the site's own, read from
its DOM; the vault has not independently checked any of the shader
implementations against their cited papers, and three of the flags
(hyperbolic, phasor, Weierstrass τ-morph) are the site marking its own
soft spots — the vault only registers that it did. The ~30 modes were
read from one page load and are presented thematically, not as a
verified index: the exact mode numbering and the substance→mode bindings
were not audited. What is certain, and is the reason the row exists, is
the posture: **an art surface that carries its own provenance and flags
its own approximations** is the same object as an `-absorbed` row, one
layer down the stack.

*object · viz.vaked.dev (ENTHEA) · ~30 sourced modes · flags where the
render is not the derivation · the constellation · fine touch from
within · vaked.dev · 8b-is, 2026-10-10*
