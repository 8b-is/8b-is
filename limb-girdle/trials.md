# trials — a curated snapshot of the LGMD trial landscape

*A v2 land, and an honest one. This is not a registry of every active LGMD
trial, "purged of nothing" — no such snapshot exists, and any table that
claims completeness is lying. What this is: a curated, reproducible snapshot
of current and selected historical LGMD interventional studies, together
with the natural-history and trial-readiness studies they lean on. Every NCT
below was pulled live from the ClinicalTrials.gov API v2 on 2026-09-24 and
keyed by the new Straub name (R/D) with the old name in parentheses — the
field has not finished renaming itself, so the table keeps both.*

## 0. Provenance and reproducibility

```
source      ClinicalTrials.gov API v2 (format=json)
retrieved   2026-09-24 (records' versionHolder 2026-09-23)
queries     query.term "limb-girdle muscular dystrophy"; "BBP-418 OR ribitol
            OR mevalonolactone"; "SRP-9003 OR SRP-9004 OR beta-sarcoglycan
            gene therapy"; "dysferlin gene therapy"; "FKRP gene therapy OR
            LION-101 OR ATA-100 OR LGMD2I"; targeted NCT lookups
            (NCT05775848, NCT04800874, NCT05973630)
fields      NCT, title, phase, overall status, lead sponsor, intervention,
            condition, study type, enrollment, primary outcomes
statuses    all shown (active, completed, terminated, unknown, withdrawn);
            historical rows are kept because programmatic attrition is part
            of the map, not noise
curation    a curated projection, not an exhaustive list; studies are
            classified by recorded study type and intervention type before
            the foundation's therapeutic taxonomy is applied
churn       phase, status, and sponsor change; this file is a snapshot, not
            a promise — re-pull before any decision that depends on it
```

The registry is split by recorded intervention type — substrate
augmentation, AAV gene replacement, then the observational / natural-history
layer — so the table can be read as *mechanism → what is actually being
tried in registered human studies.* Transcript repair (exon skipping) and
genome editing are not separately sectioned here because no registered
interventional LGMD trial of those types was captured in this snapshot; they
remain predominantly preclinical (see [`foundation.md`](foundation.md)).

## 1. Substrate / metabolite augmentation — the metabolic hypothesis

| NCT | subtype (new → old) | gene | intervention | sponsor | phase | status |
|---|---|---|---|---|---|---|
| [NCT05775848](https://clinicaltrials.gov/study/NCT05775848) · Fortify · MLB-01-005 | R9 → 2I | FKRP | BBP-418 (ribitol) vs placebo | ML Bio Solutions | 3 | Active, not recruiting |
| [NCT07678775](https://clinicaltrials.gov/study/NCT07678775) | R9 → 2I | FKRP | BBP-418 (ribitol), open-label extension | ML Bio Solutions | 3 | Active, not recruiting |
| [NCT04800874](https://clinicaltrials.gov/study/NCT04800874) · MLB-01-003 | R9 → 2I | FKRP | BBP-418 (ribitol) | ML Bio Solutions | 2 | Active, not recruiting |

*Substrate augmentation in trial form. Pathogenic FKRP variants impair the
CDP-ribitol pathway; the trial enlarges the substrate pool rather than
repairing the enzyme. Investigational — the metabolic hypothesis now in
Phase 3 testing, not an established treatment.*

**The flagship — FORTIFY (MLB-01-005).** Phase 3, randomized 2:1,
placebo-controlled, quadruple-masked, 112 enrolled, ages 12–60. Primary:
North Star Assessment for LGMD (NSAD) change at 36 months, plus safety. Key
secondaries: 10 m walk velocity, FVC, and PUL 2.0. Total glycosylated
α-dystroglycan (αDG) and serum CK are registered as exploratory ("other")
outcomes — mechanistically aligned, but not the primary evidentiary burden.
NSAD and safety carry that.

## 2. AAV gene replacement — the structural hypothesis

### Sarcoglycanopathies (R3–R6)

| NCT | subtype (new → old) | gene | intervention | sponsor | phase | status |
|---|---|---|---|---|---|---|
| [NCT06246513](https://clinicaltrials.gov/study/NCT06246513) | R4 → 2E | SGCB | SRP-9003 (bidridistrogene xeboparvovec) | Sarepta | 3 | Active, not recruiting |
| [NCT05876780](https://clinicaltrials.gov/study/NCT05876780) | R4 → 2E | SGCB | SRP-9003 | Sarepta | 1 | Active, not recruiting |
| [NCT03652259](https://clinicaltrials.gov/study/NCT03652259) | R4 → 2E | SGCB | SRP-9003 | Sarepta | 1/2 | Terminated |
| [NCT01976091](https://clinicaltrials.gov/study/NCT01976091) | R3 → 2D | SGCA | SRP-9004 (patidistrogene bexoparvovec) | Sarepta | 1/2 | Completed |
| [NCT06747273](https://clinicaltrials.gov/study/NCT06747273) | R3 → 2D | SGCA | SRP-9004, systemic infusion | Sarepta | 1 | Terminated |
| [NCT00494195](https://clinicaltrials.gov/study/NCT00494195) | R3 → 2D | SGCA | rAAV1.tMCK.hαSG (IM) | Nationwide Children's | 1 | Completed |
| [NCT05973630](https://clinicaltrials.gov/study/NCT05973630) · ATA-003-GSAR | R5 → 2C | SGCG | ATA-200 (AAV8-SGCG, IV) | Atamyo | 1 | Active, not recruiting |

### Dysferlinopathy (R2) — the size constraint

| NCT | subtype (new → old) | gene | intervention | sponsor | phase | status |
|---|---|---|---|---|---|---|
| [NCT05906251](https://clinicaltrials.gov/study/NCT05906251) | R2 → 2B | DYSF | SRP-6004 | Sarepta | 1 | Terminated |
| [NCT02710500](https://clinicaltrials.gov/study/NCT02710500) | R2 → 2B | DYSF | rAAVrh74.MHCK7.DYSF.DV (dual vector) | Sarepta | 1 | Completed |

*Dysferlin's ~7 kb coding sequence presses the single-AAV ceiling; the
dual-vector construct (`.DV`) is one answer to that size constraint. Both
registered programs here are early and have ended (one completed, one
terminated). The size constraint is a real design pressure, but these records
do not attribute their closure to it.*

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
| [NCT04202627](https://clinicaltrials.gov/study/NCT04202627) | R9 → 2I | biomarker development (observational, no drug) | ML Bio Solutions | Completed |
| [NCT03981289](https://clinicaltrials.gov/study/NCT03981289) | LGMD broad | defining clinical endpoints | VCU | Completed |
| [NCT03488784](https://clinicaltrials.gov/study/NCT03488784) | R1 + R4 | natural history | Lindsay Alfano | Completed |
| [NCT01676077](https://clinicaltrials.gov/study/NCT01676077) | R2 (dysferlinopathy) | clinical outcome | Newcastle | Unknown |
| [NCT03842878](https://clinicaltrials.gov/study/NCT03842878) | R9 → 2I | natural history | Genethon | Completed |

## 4. Read the registry honestly

Three things fall out of the table when you stop and read it:

1. **Two strategies have reached Phase 3, not one.** Ribitol (R9) and
   SRP-9003 (R4) both hold Phase 3 records. Phase 3 indicates developmental
   maturity, not comparative probability of approval.
2. **Programs have stopped, and the records say why.** The terminated
   sarcoglycan and dysferlin trials were closed for sponsor-reported business
   reasons. They demonstrate programmatic attrition, not failure attributable
   to a specified biological mechanism.
3. **A matching gene is not a matching trial.** Eligibility also turns on the
   precise variant, age, ambulatory status, functional range, cardiac and
   respiratory criteria, prior therapies, anti-AAV antibodies, and geography.
   Naming the subtype narrows the search; it does not select the row.

The sound conclusion is narrower and better:

> This snapshot shows which mechanistic strategies have entered registered
> human studies, which programs remain active, and where development has
> stopped. It does not establish why a program stopped, whether an
> intervention works, how close it is to approval, or whether any individual
> is eligible.

*the trial registry · curated snapshot · NCT · SRP-9003 · SRP-9004 · ATA-200 ·
BBP-418 ribitol · AB-1003 LION-101 · ATA-100 · sarcoglycanopathy ·
dysferlinopathy · FKRP · the ledger · v2 · the constellation · 8b-is, 2026-09-24*
