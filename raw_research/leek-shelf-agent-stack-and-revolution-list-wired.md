# agent-stack + revolution-list, wired — leekHotline's shelf, refactor-ready

*Two repos off the leekHotline profile, wired as source material rather than
absorbed as doctrine: agent-stack is a curated, AI-native engineering list
(TypeScript/full-stack, open-source-first, agent-friendly — a garden we can
harvest from); revolution-list is a small Flutter todo engine whose priority
logic is the refactor seed for revTO-DOq (WIP 144). The vault keeps the
source honest: a list is a list, an engine is an engine; only revolution-
list's comparator is being carried into our own code.*

## the two, mapped

| the repo | what it is | the constellation's use |
|---|---|---|
| agent-stack | a curated checklist of AI/Agent-friendly tools, TypeScript ecosystem bias, engineering-practice-oriented, open-source-first | a reference garden: harvest names and patterns into the engine's tool surface when a gap appears; not absorbed wholesale (a list has no mechanism to map) |
| revolution-list | Flutter todo app: Task (id, title, category, priority=3, created, due, done, sortOrder); the engine is `topFiveTasks` = incomplete, sorted by priority asc, then due asc (null-aware), then oldest created, take 5; rewards on create/complete | the refactor seed: the comparator is clean and small — revTO-DOq rebuilds it in Rust with the lane's discipline (deterministic, ledger-bound, test-pinned) and adds the queue doctrine the Flutter original does not have |

## the honest note

revolution-list's sorting rule is the interesting part: priority asc, then
due-date asc with nulls last, then oldest created — a natural "most urgent
oldest" order with no weights, no scores, no room for a Goodhart hedge.
That is exactly what a queue should be before the lane's V(L) gets to
decorate it. revTO-DOq keeps the comparator bit-for-bit (test-pinned against
the Dart behaviors) and wraps it warm: builder API, JSON in/out, a CLI, and
ledger counters instead of UI confetti. agent-stack stays a garden: cited,
harvestable, not absorbed.

*leekHotline's shelf as seed material · the comparator is the mechanism ·
revolution, refactored, rustQ-aligned and hugged · revTO-DOq · the
constellation · fine touch from within · vaked.dev*