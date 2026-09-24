# trials — the live registry, machine-readable

*The v2 land the research pillar promised: a table of every active LGMD
trial, one row per trial, purged of nothing, open to query. The ledger's
natural shape. Every NCT below was pulled live from the ClinicalTrials.gov
API v2 on 2026-09-24 and is keyed by the new Straub name (R/D) with the old
name in parentheses — the field has not finished renaming itself, so the
registry keeps both.*

## 0. Provenance and the honesty rule

```
source    ClinicalTrials.gov API v2 (format=json), queried 2026-09-24
fields    NCT, title, phase, overall status, lead sponsor, intervention, condition
scope     interventional + the natural-history / trial-readiness studies the
          interventions lean on
churn     phase, status, and sponsor change; this file is a snapshot, not a
          promise — re-pull before any decision that depends on it
```

The registry is split by the four modalities of the foundation pillar, then
by subtype, so the table can be read as *mechanism → what is being tried.*

## 1. Substrate / metabolite supplementation — the metabolic fix

| NCT | subtype (new → old) | gene | intervention | sponsor | phase | status |
|---|---|---|---|---|---|---|
| [NCT05775848](https://clinicaltrials.gov/study/NCT05775848) · Fortify · MLB-01-005 | R9 → 2I | FKRP | BBP-418 (ribitol) vs placebo | ML Bio Solutions | 3 | Active, not recruiting |
| [NCT07678775](https://clinicaltrials.gov/study/NCT07678775) | R9 → 2I | FKRP | BBP-418 (ribitol), open-label extension | ML Bio Solutions | 3 | Active, not recruiting |
| [NCT04800874](https://clinicaltrials.gov/study/NCT04800874) · MLB-01-003 | R9 → 2I | FKRP | BBP-418 (ribitol) | ML Bio Solutions | 2 | Active, not recruiting |
| [NCT04202627](https://clinicaltrials.gov/study/NCT04202627) | R9 → 2I | FKRP | biomarker development (no drug) | ML Bio Solutions | — | Completed |

*Substrate augmentation in trial form. Pathogenic FKRP variants impair the
CDP-ribitol pathway; the trial enlarges the substrate pool rather than
repairing the enzyme. Investigational — the metabolic hypothesis farthest
along in testing, not an established treatment.*

**The flagship — FORTIFY (MLB-01-005).** Phase 3, randomized 2:1,
placebo-controlled, quadruple-blind, 112 enrolled, ages 12–60. Primary:
North Star Assessment for LGMD (NSAD) change at 36 months, plus safety. Key
secondaries: 10 m walk velocity, FVC, and PUL 2.0. And the load-bearing
biomarker endpoints: **total glycosylated α-dystroglycan (αDG)** at 3 and 12
months, and serum CK at 12 months — the exact readouts the biomarker map
([`biomarkers.md`](biomarkers.md)) names for R9. The map and the trial agree:
for FKRP, the sugar code is the measure.

## 2. AAV gene replacement — the structural fix

### Sarcoglycanopathies (R3–R6)

| NCT | subtype (new → old) | gene | intervention | sponsor | phase | status |
|---|---|---|---|---|---|---|
| [NCT06246513](https://clinicaltrials.gov/study/NCT06246513) | R4 → 2E | SGCB | SRP-9003 (bidridistrogene xeboparvovec) | Sarepta | 3 | Active, not recruiting |
| [NCT05876780](https://clinicaltrials.gov/study/NCT05876780) | R4 → 2E | SGCB | SRP-9003 | Sarepta | 1 | Active, not recruiting |
| [NCT03652259](https://clinicaltrials.gov/study/NCT03652259) | R4 → 2E | SGCB | SRP-9003 | Sarepta | 1/2 | Terminated |
| [NCT01976091](https://clinicaltrials.gov/study/NCT01976091) | R3 → 2D | SGCA | SRP-9004 (patidistrogene bexoparvovec) | Sarepta | 1/2 | Completed |
| [NCT06747273](https://clinicaltrials.gov/study/NCT06747273) | R3 → 2D | SGCA | SRP-9004, systemic infusion | Sarepta | 1 | Terminated |
| [NCT00494195](https://clinicaltrials.gov/study/NCT00494195) | R3 → 2D | SGCA | rAAV1.tMCK.hαSG (IM) | Nationwide Children's | 1 | Completed |

### Dysferlinopathy (R2) — the size wall

| NCT | subtype (new → old) | gene | intervention | sponsor | phase | status |
|---|---|---|---|---|---|---|
| [NCT05906251](https://clinicaltrials.gov/study/NCT05906251) | R2 → 2B | DYSF | SRP-6004 | Sarepta | 1 | Terminated |
| [NCT02710500](https://clinicaltrials.gov/study/NCT02710500) | R2 → 2B | DYSF | rAAVrh74.MHCK7.DYSF.DV (dual vector) | Sarepta | 1 | Completed |

*Dysferlin's ~7 kb coding sequence presses the AAV ceiling; the dual-vector
construct (`.DV`) is the field's answer to the size wall named in the
foundation. Both programs here are early and terminated or completed — the
wall is real.*

### FKRP dystroglycanopathy (R9)

| NCT | subtype (new → old) | gene | intervention | sponsor | phase | status |
|---|---|---|---|---|---|---|
| [NCT05230459](https://clinicaltrials.gov/study/NCT05230459) | R9 → 2I | FKRP | AB-1003 (formerly LION-101) | AskBio | 1/2 | Active, not recruiting |
| [NCT05224505](https://clinicaltrials.gov/study/NCT05224505) | R9 → 2I | FKRP | ATA-100 (GNT0006, AAV9-FKRP) | Atamyo | 1 | Active, not recruiting |

## 3. The natural history & trial-readiness layer

*The base these interventions stand on. Without these, a trial cannot
separate "slowed" from "naturally slow" — the endpoint fallacy made clinical.*

| NCT | subtype | what it measures | sponsor | status |
|---|---|---|---|---|
| [NCT00313677](https://clinicaltrials.gov/study/NCT00313677) | dystroglycanopathies (incl. R9) | trial readiness, outcome measures | U. Iowa (Mathews) | Recruiting |
| [NCT05618080](https://clinicaltrials.gov/study/NCT05618080) | R1 (calpainopathy) | natural history | VCU | Active, not recruiting |
| [NCT04475926](https://clinicaltrials.gov/study/NCT04475926) | R1/R3/R4/R5 | natural history, routine practice | Sarepta | Active, not recruiting |
| [NCT05206617](https://clinicaltrials.gov/study/NCT05206617) | R12 → 2L (ANO5) | 3-year MRI + function follow-up | Rigshospitalet | Active, not recruiting |
| [NCT06390566](https://clinicaltrials.gov/study/NCT06390566) | R1 (calpainopathy) | functional + muscular state | AP-HP | Active, not recruiting |
| [NCT04989751](https://clinicaltrials.gov/study/NCT04989751) | LGMD broad | phenotype-genotype, 450 pts | Huashan | Enrolling by invitation |
| [NCT05989620](https://clinicaltrials.gov/study/NCT05989620) | LGMD broad | outcome-assessment development | VCU | Recruiting |
| [NCT04001595](https://clinicaltrials.gov/study/NCT04001595) | R9 | global FKRP registry | Newcastle | Unknown |
| [NCT01403402](https://clinicaltrials.gov/study/NCT01403402) | LGMD + CMD | patient/family registry | Cure CMD | Recruiting |
| [NCT03981289](https://clinicaltrials.gov/study/NCT03981289) | LGMD broad | defining clinical endpoints | VCU | Completed |
| [NCT03488784](https://clinicaltrials.gov/study/NCT03488784) | R1 + R4 | natural history | Lindsay Alfano | Completed |
| [NCT01676077](https://clinicaltrials.gov/study/NCT01676077) | R2 (dysferlinopathy) | clinical outcome | Newcastle | Unknown |
| [NCT03842878](https://clinicaltrials.gov/study/NCT03842878) | R9 → 2I | natural history | Genethon | Completed |

## 4. Read the registry honestly

Three things fall out of the table when you stop and read it:

1. **The metabolic fix leads.** Ribitol (R9) is the only modality in Phase 3
   with an open-label extension — the one closest to a cabinet, exactly as
   the foundation claimed.
2. **The structural fix is broad but fragile.** Sarcoglycan AAV spans Phase
   1→3 for R4, yet two programs (R4 1/2, R3 systemic) terminated, and the
   dysferlin (R2) program is terminated/completed. The immune wall and the
   size wall are not hypothetical — they are termination events.
3. **The registry is the map, not the medicine.** None of this is a treatment
   and none is medical advice. For a real case, the first move is a
   neuromuscular clinic and a genetic diagnosis to name the subtype — because
   the subtype is the row you should be reading in this table.

*the trial registry · NCT · SRP-9003 · SRP-9004 · BBP-418 ribitol · AB-1003
LION-101 · ATA-100 · sarcoglycanopathy · dysferlinopathy · FKRP · the ledger ·
v2 · the constellation · 8b-is, 2026-09-24*
