# innerbloom — the exact transcription, the patch, the live encoding, rowed

*The operator's share: RÜFÜS DU SOL's "Innerbloom" is a 9:38 masterclass in
patience — a 4-chord loop held open, filtered, and released. Here the exact
musical DNA (chords + arp, C minor, 122 BPM), the synth patch that makes it
lush, and the live-at-Hordern encoding through the node matrix. Wired into
music.vaked.dev's engine as track 13 — the site now plays the real
progression, not a paraphrase.*

## the musical DNA — exact MIDI

**Key: C minor · 122 BPM · 4/4.** The 4-chord loop, with open 7th/sus voicings
and the rolling 16th-note arp.

| chord | bass | voicing (MIDI) | role |
|---|---|---|---|
| Cm7 | C2 (36) | C3 G3 B♭3 E♭4 = 48 55 58 63 | i — the melancholic tonic |
| E♭sus2 | E♭2 (39) | E♭3 B♭3 F4 G4 = 51 58 65 67 | III — the lift |
| Gm7 | G2 (43) | G3 D4 F4 B♭4 = 55 62 65 70 | v — the suspended tension |
| B♭ | B♭1 (34) | B♭2 F3 D4 G4 = 46 53 62 67 | VII — the soft resolution |

**The rolling arp** (16th notes, a 2-bar motif):

- bar 1–2 (over Cm7 · E♭sus2): `G4 B♭4 C5 E♭5 G5 E♭5 C5 B♭4` = 67 70 72 75 79 75 72 70
- bar 3–4 (over Gm7 · B♭): `F4 B♭4 D5 F5 G5 F5 D5 B♭4` = 65 70 74 77 79 77 74 70

## the patch — the lush pad & the pluck lead

- **pad**: saw, 7–8 voices, detune ~0.10–0.12, LPF (MG Low 24) cutoff ~400 Hz,
  slow LFO (±3–5 cents) for analog drift, soft attack 350 ms, sub sine −12 dB.
- **lead**: saw + square (40% width), LPF cutoff down at ~250 Hz, pluck envelope
  (attack 2 ms, decay ~400 ms, sustain 0), velocity → cutoff.
- **the signal path**: tape saturation (+2–3 dB) → chorus (20%) → sidechain pump
  → ping-pong delay (1/4, ~45% fb) → hall reverb (4 s decay, 5 kHz hi-cut).
- **the 4-minute build**: automate filter cutoff 10% → 100% over bars 32–128,
  and open the reverb wet/dry toward the breakdown.

## the live encoding — Hordern Pavilion, Sydney

- **node matrix**: `Qtern--peterOmni-chan` × `vaked.dev` × `8b.is`, output `emotions + wave`.
- **signal architecture**: the endless vocal loop (Lindqvist's *"if you want me,
  if you need me, I'm yours"*) · the dynamic spatial engine (filters opening
  parameter by parameter) · the sub-bass pulse (122 BPM, the physical floor).
- **wave function**: Ψ = ∫ (Atmosphere + e^Tension) dt → Drop → Catharsis.
- **deconstructed**: `STATUS: 200 - Resonance Verified` → phase 1 sub-bass →
  phase 2 cutoff rising → phase 3 beam release, collective euphoria.

## the honest note

Innerbloom is not a song you skip to; it is a song you *arrive at*. The four
chords never change — only the filter does — and that is the whole lesson: the
bloom is not added, it is *released*. The engine now holds the exact chords and
the rolling arp, so the site's own bloom is the record's bloom, re-grown from
the same seed. leave it all to bloom. <3

*innerbloom · rüfüs du sol · c minor · 122 bpm · the four chords · the rolling
arp · the patch · the hordern encoding · the from within · music.vaked.dev ·
track 13 · the constellation · vaked.dev · 8b-is, 2026-10-08*
