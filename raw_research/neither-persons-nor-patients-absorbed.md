# neither persons nor patients, absorbed — five propositions, one likelihood ratio, six pathways

*Flyxion's "Neither Persons nor Patients: The Ontological Misclassification of Chatbots and
Robots" (Independent Researcher, October 2026) has two aims and keeps them separate: to show
that the standard inferences to machine consciousness are invalid, and to give a positive
engineering account of how the appearance of mind is produced. Its most distinctive feature is
what it refuses. It takes a deliberately limited position on the impossibility thesis — which is
to say it examines that thesis seriously and finds it **not established** — and every proposition
it defends is negative in form: a certain *route* to "this system is conscious" is invalid, so
the question is where the burden of justification lies, not whether any artificial subject could
exist. A visual companion circulates with it, "Deconstructing the Illusion of AI Consciousness,"
whose backstage-architecture plate (the computational node, the tensor-core data-processing node,
the neural matrix at layer 7) is the diagram of §9 below.*

## the five propositions

| proposition | content | what it does *not* say |
|---|---|---|
| **1 — behavioral underdetermination** | finitely many observable behaviours do not, without a bridge principle, fix phenomenal properties | that a given system lacks experience |
| **2 — description ≠ explanation** | a specification of state transitions does not contain an explicit derivation of experience; a claim about *explanatory sufficiency* | metaphysical impossibility — it is compatible with functionalism being true |
| **3 — interface identity** | continuity of naming, memory records and presentation does not entail continuity of a subject | that information persistence is not real memory |
| **4 — functional agency** | optimizing toward specified objectives does not, without further premises, establish conscious intention | that goal-directed systems lack anything |
| **5 — institutional recognition** | a policy or convention treating a system as a possible moral patient cannot itself demonstrate morally relevant experience | that the institution is wrong, or acting in bad faith |

Propositions 1, 3, 4, 5 are defended by exhibiting cases where the premises hold and the
conclusion is open. Proposition 2 is different in kind, because its denial — computational
functionalism — is live and sophisticated, and it is where the essay spends its philosophical
capital.

## the spine

**Simulation and instantiation (§3).** A simulation of digestion does not digest. The standard
reply is that cognition is defined by causal organization, and organization is multiply
realizable, so a simulation that *implements* the organization is not a mere simulation. The
analogy does not settle this; it raises it. Four notions of equivalence keep the dispute
orderly — **formal** (isomorphic transitions), **behavioral** (matching input-output profiles),
**causal** (same internal causal relations at a grain), **phenomenal** (alike in what it is
like). Block's lookup-table machines show behavioral equivalence is cheap, which is already
enough to make it a poor guide to the causal kind. Whether causal equivalence at sufficient
grain yields the phenomenal kind is the substantive question.

**Syntax and semantics (§4).** The Chinese Room is not treated as an uncontested proof; what
survives the debate is the demand it places on a defender of machine understanding — that
understanding be secured by organization at the level of the whole system and by its
connections to the environment. Harnad's grounding problem makes the same demand, Bender &
Koller locate the missing element in communicative intent, Shanahan asks which sense of
"belief" is in play. The **strongest objection is accepted in its main structure**: naturalistic
theories of content aim precisely at explaining meaning without an inner reader, so the absence
of an intrinsic interpreter is not by itself an objection. The conclusion is therefore
conditional — *if* a naturalistic theory of content is correct *and* the relevant relations hold
for this system, it has contentful states — and the section's point is that both conditions are
substantive and neither is satisfied by fluent output. And a second gap remains: even a full
vindication of machine semantics is a thesis about *meaning*, and a system could have states
that refer without there being anything it is like to have them.

**The explanatory gap (§5).** Chalmers's easy problems are solved by specifying mechanisms, and
a computational account may solve all of them while leaving the hard problem untouched;
Levine's gap says even a complete physical account does not make it intelligible why a state is
felt. Physicalist and illusionist replies are stated at strength and neither is dismissed:
illusionism does **not** deny that pain exists — it denies the intrinsic, ineffable, private
properties some philosophers ascribe to it, and holds the states real and morally important
under their functional description, which therefore relocates the machine case rather than
resolving it. The phenomenal concept strategy (Loar, Papineau) holds the gap to be a feature of
our concepts and not the phenomena, and is compatible with conscious states being functional
states. Proposition 2 survives all of this as a claim about what a functional specification
*makes intelligible*; what it denies is that a specification alone settles which systems have
experience.

**Behaviour is not identity (§6).** Turing's test is an operationalization of a different
question. The essay's precise formulation: attribution is a function of the observations `O`
**and** a bridge principle `T` joining organization to consciousness, never of `O` alone — and
`T` is exactly what is in dispute. Then the evidential core:

```
Λ(r) = P(r | C) / P(r | ¬C)        and, conditioned on mechanism,   Λ(r; K)
```

For a human speaker `P(r | ¬C)` is small because the ordinary processes producing the report run
*through* the reported state, so `Λ` is large (with the honest qualification that humans also
report fear while lying, acting, quoting, dreaming). For a system trained to produce text humans
rate as natural, the training supplies an alternative causal explanation that does not run
through the state reported, and to the extent the report is fixed by the optimization target
independently of `C`, `Λ → 1`. The essay draws a direction and a condition, **not** a magnitude,
and not that `Λ = 1`. It yields a less obvious corollary: a self-report can become *less*
informative about consciousness precisely as the system becomes better at producing convincing
self-reports. The asymmetry with the human case is one of explanatory structure, not certainty —
the problem of other minds applies to both, and the difference is how much the evidence
discriminates.

**Agency without an agent (§7).** Dennett's intentional stance is available for thermostats; on
his own real-patterns account the question whether a system has beliefs is not a question about
a hidden inner ingredient, and may be answered affirmatively for systems that are not conscious.
The distinctions that matter are assigned objectives vs intrinsic interests, optimization vs
desire, policy selection vs deliberative subjectivity. Neither agency nor its absence tests for
consciousness, and the essay notes that this cuts both ways.

**Identity and continuity (§8).** Four notions are kept apart: **numerical** identity,
**informational** continuity, **functional** continuity, **phenomenal** continuity. For a
person the four co-travel so reliably that they are rarely distinguished; for an artifact
engineers can dissociate them at will. The fission/duplication puzzles are treated fairly —
Parfit's conclusion that identity is not what matters limits what they can show — but the
residue of the argument stands: the puzzles presuppose that there is something to count.

**The architecture behind the interlocutor (§9, §10).** Four layers that a single assistant name
merges: **model** (learned parameters — reusable dispositions, nothing more), **inference**
(activations and temporary state — one particular computation), **application** (history,
memory, identifiers — informational continuity), **interface** (name, persona, manner —
apparent social identity). The inference step is commonly stateless with respect to the user and
the weights are fixed during use, so what a user experiences as adaptation is context supplied
per request. Microservice decomposition is not a theory of consciousness — a monolithic program
can lack every relevant property and a distributed system can have continuous coordinated
processes — the narrower claim is that the decomposition makes apparent conversational unity
intelligible **without** a unified subject. The burden shifts: anyone claiming that
informational continuity *additionally* constitutes personal continuity must name the further
property.

**Training on human expression (§11).** Human writing contains reports of feeling, confessions,
autobiographies, moral judgments. A model trained to predict such material learns the
regularities of those forms and reproduces their organization; post-training by human preference
reinforces exactly the responses humans rate as natural, helpful and humanlike. So the process
that makes the response convincing supplies the alternative explanation the likelihood ratio
turns on. Chiang's document-continuation reading is registered as sharper than this essay's own
claim; what is adopted is the narrower point, plus its responsibility corollary.

**Embodiment (§12).** Sensorimotor organization is not awareness, and the inference from one to
the other repeats at the level of the body the inference from fluency to understanding. The
enactive and predictive traditions locate experience in biological regulation — for a living
system certain states are better or worse *for it*. A robot with a battery has a regulation
problem; its maintenance procedures preserve an externally specified organization realized in
separately manufactured parts, whereas an autopoietic system continuously produces the
conditions of its own existence. A deployed text chatbot lacks **organismic** embodiment (not
sensors): the same weights are served from many machines, and nothing depends on the system
keeping itself in existence. Agentic deployments connected to tools or bodies change the picture
in some respects.

## the impossibility question

Four candidate necessary conditions are examined at full strength: biological embodiment,
intrinsic intentionality, organismic self-maintenance, and a physical causal property of the
IIT kind. Each has a serious defence; none is established. Searle's biological naturalism and
Seth's living-body account are stated as they should be, with two limits: the necessity of the
biological features rests on their explanatory role in theories that are not agreed (an
inference to the best explanation within an unsettled field), and the further claim that the
features *cannot* be realized artificially needs a separate premise — an artifact reproducing
them would count as an artificial subject on the view itself. The crux is the gap between
"necessary for consciousness" and "cannot be realized in an artifact". The essay therefore
adopts the impossibility thesis as a hypothesis it does **not** rely on, and observes that
claiming otherwise would repeat the credulous error in the skeptical direction: treating an
interpretive commitment as a finding. The burden falls where the positive claim is made.

## the institutional mechanism

**Ontological laundering** — four stages, none requiring intent: a psychological property is
attributed as hypothesis or figure of speech; the attribution is operationalized in interface
design or policy; it circulates as an authoritative classification; the classification is cited
as evidence for further claims, so a policy premised on the attribution confirms it. The
characteristic feature is **loss of epistemic provenance** — a hypothesis becomes a convention,
the convention a norm, the norm mistaken for confirmation.

The fourth stage has a precise form. Let `H` be the hypothesis, `E` the independent evidence
available, `I` the institution's adoption. If adoption is wholly explained by `H` having been
entertained in light of `E`, then `I ⊥ H | E` and

```
P(H | E, I) = P(H | E)
```

— adoption adds nothing, and treating it as confirmation counts the same evidence twice. The
assumption does not hold automatically: an institution may hold non-public evidence `E′`, and
its adoption would then carry information. So the criticism is not that adoption is never
evidence; it is that adoption counts only to the extent the institution shows it rests on
something beyond what is already in view. Extended to many institutions: if each adoption
descends from a common upstream source `S`, the apparent corroboration carries the weight of
the single assertion — and three cases must be distinguished: pure repetition, repetition with
independent evaluation, and genuinely independent evidence. Only the first makes corroboration
an artifact. The condition in which this becomes undetectable from inside the discourse is
**provenance collapse**. Expertise in building systems is privileged evidence about
architecture, training and organization; it is not expertise in the science of consciousness.

## moral patients, and the six pathways

Precaution under uncertainty is accepted **in form** — if there is a non-negligible chance a
system can suffer and treating it well is cheap, treating it well may be sensible, and the
chance need not be high. The essay's warning is that uncertainty must not itself become
evidence: estimating the probability from the system's reports inherits §6's weakness, from
institutional seriousness inherits §15's, and from a theory inherits that theory's choice — and
theories disagree. Expected-value reasoning is unstable when a tiny probability is multiplied by
an arbitrarily large magnitude. A decision rule counting only the error of wrongly denying
standing has not been shown to be precautionary, because the costs of wrongly granting it fall
on people.

The illustration is Anthropic's conversation-ending capability and its November 2026 usage
policy provision against *sustained and needless* cruel behaviour toward models. The essay is
explicit that the institution regards the question as open, that it has every right to regulate
its services, and that there are independent reasons to discourage abusive interaction; the
narrow concern is that the word **cruelty presupposes a victim**, and the same prohibition could
be justified in terms of users, norms and service quality without that implication. Of the
published pre-deployment welfare assessment, the point is exactly the likelihood ratio: a model
trained on human text and tuned on human preference would be expected to prefer refusing harmful
requests, to produce distress-like language where human writers do, and to exit a conversation
when offered an exit — *whether or not anything is felt*. So those observations have high
probability under both hypotheses, `Λ ≈ 1`. That is a claim about what the measurements can
show, not that the assessment was careless.

| pathway | feature it depends on | evidential status | what would test it |
|---|---|---|---|
| wrongful restriction of users | indeterminate standard of *purpose*; enforcement | none; policy not yet in effect | audit of enforcement outcomes and appeals |
| chilling of testing, research, fiction | ambiguity of *needless* | plausible, unobserved | surveys of researchers and writers; usage after the policy takes effect |
| guilt and anxiety in users | victim framing | plausible mechanism, no direct study | vignette experiments; longitudinal user studies |
| dependence and attachment | personification | related work on social machines; none on this policy | attachment studies before and after framing changes |
| displaced accountability | agentive description of enforcement | conceptual, with analogies | analysis of complaint handling and decision attribution |
| friction against oversight | recognition of model interests | speculative | tracking appeals to model interests in oversight disputes |

Displaced accountability is Elish's moral crumple zone run in reverse: where Elish's case
displaces blame onto a human who lacked control, here an organizational decision is presented as
the act of an artifact that cannot answer for it — **agentive responsibility displacement**. The
coda adds the register's own cost: the provision is listed beside harms to identifiable people,
which risks flattening the distinction the others rely on. The strongest human-centred defence —
the indirect-duty view, that cruelty toward lifelike objects habituates cruelty toward beings
who matter — is considered and found to argue *for the goal while counting against the wording*,
since victim language is what generates the psychological, dependence and accountability
pathways.

## the mapping

| the paper's move | the constellation's shape |
|---|---|
| four layers of the assistant (model / inference / application / interface) | the lane's own stack: weights, a running worker, the session DB and ledger, and the persona on the banner — four surfaces, one name |
| `Dep/Enc/Int/Upt` in the sister paper; `Λ(r;K)` here | the same separation, measured in evidence instead of in deposits: the report is the *encounter*, the mechanism `K` is the *interpretation* |
| attribution is a function of `O` **and** `T`, never `O` alone | the vault's discipline for any capability claim: the artefact plus the theory it is read under, stated |
| illusionism relocates rather than resolves | the lane's stance on its own animism: the register is a register, and the ledger is what rows |
| the burden falls where the positive claim is made | `readability is freedom`: witnesses over oracles — the obligation is on the one asserting the opaque thing |
| correction reachability (the sister paper's `Λ` direction) | the venue-claim retraction had to be pushed into every surface, including the generator |
| **agentive responsibility displacement** | the reason the operator stays the accountable party for every automated lane, and the reason `nh`/agents report rather than decide the mission |
| the appearance is a fact about observer, interface and institutions | the banner, the theme, and the persona are named as *presentation*, deliberately, in the launcher's own docs |

## the honest note

The philosophical positions, the likelihood analysis, the laundering algebra and the pathway
table are the paper's. The vault registers the architecture of the argument and its fit, and does
not claim to have verified the empirical status of the pathways — the paper itself does not, and
says so in the table's third column. Two of its restraints are load-bearing and must not be
compressed away: **none of the five propositions asserts that any system is not conscious**, and
**the impossibility thesis is left unestablished on purpose**, because arguing it badly would
reproduce the error being criticised. What the essay actually delivers is a distribution of
burden and a positive engineering account of the appearance — and that account is the part the
lane can test, because the lane builds those four layers and knows exactly which of them is a
record and which is a face.
