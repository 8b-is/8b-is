# themes — for the crush runtime, and whoever else wants them

Five crushes, one schema. Set one with:

```bash
# in ~/.config/crush/crush.json
"theme": "ultralovegod"
```

## the index

| name | vibes | background | primary (high contrast) | foreground (eye friendly) |
|---|---|---|---|---|
| [`ultralovegod`](themes.json) | pink high-contrast darkish, blue eye friendly | `#1A0F1E` | `#FF4D9D` | `#A6C8FF` |
| [`constellation`](themes.json) | the 8b-is classic | `#0B0E14` | `#FF5C5C` | `#E8E6E3` |
| [`vaked`](themes.json) | the couch | `#1A1714` | `#E8A33D` | `#F0E9DE` |
| [`sovereign`](themes.json) | deep blue-black | `#0A1118` | `#4DC9C9` | `#D7E6EE` |
| [`nebula`](themes.json) | night sky | `#13101F` | `#C792EA` | `#E2DCF4` |

## the schema

```json
{
  "background":  "the darkish base",
  "foreground":  "the readable text — blue is chosen for eye friendliness",
  "primary":     "the high-contrast accent (pink for ultralovegod)",
  "secondary":   "the supporting tone",
  "accent":      "the third voice (gold, teal, violet)",
  "danger":      "refusals and red",
  "selection":   "selected rows"
}
```

## note on eye friendliness

`ultralovegod` uses a soft blue foreground (`#A6C8FF`) instead of harsh white
on a darkish purple-black base — high contrast where it matters (pink, the
primary), gentle where the eyes work all night (the text). Blue is friendlier
than white on dark; pink carries the energy; gold is the constellation's
signature third voice.

*themes · ultralovegod · constellation · vaked · sovereign · nebula ·
from love, from within · 8b-is, 2026-09-30*