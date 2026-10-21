# Progress

Agent-maintained state. Update the **Current cycle** block the moment a step finishes, so `/lesson` can resume after an interruption.

Course: https://school.bondiano.com/r/23-rust — lesson URL: `https://school.bondiano.com/l/23-rust/<slug>`.
Milestone: lessons 01–17.

## Current cycle

- lesson: 04
- step: review
- homework: — (parked: CONFLICT, SPLIT and EXCLUSIVE write-ups; E-006 halves range entry awaiting learner wording)
- pre-drill done: yes
- last warm-up: 2026-10-19

Steps, in order: `warm-up` → `pre-drill` → `reading` → `homework` → `feynman` → `review` → `quiz` → `close`.

## Lessons

Completion criteria (all four, or the learner declares the lesson completed):
1. red tests of every homework green, `just check` clean;
2. Feynman explanation with no open gaps;
3. on-site quiz passed (learner confirms);
4. ≥2 of 3 warm-up questions on this lesson answered without hints in the next session.

| # | slug | crate | homework green | feynman | quiz | warm-up | status |
|---|------|-------|----------------|---------|------|---------|--------|
| 01 | 01-why-rust-setup | l-01-setup | — | — | — | — | completed (before the system) |
| 02 | 02-basic-types-control-flow | l-02-basics | FIZZBUZZ, LOOP-VALUE red: solution never committed | — | — | ✓ (2/3, 2026-10-18) | completed (before the system) |
| 03 | 03-ownership-move-copy | l-03-ownership | FIX-MOVE ✓, GUARD ✓, PREDICT ✓, WIDGET ✓ (declared done by learner) | ✓ (oral) | ✓ | ✗ | awaiting warm-up criterion (next sitting) |
| 04 | 04-borrowing-references-slices | l-04-borrowing | FIRST-WORD ✓, SPLIT ✓ (code, rung 4); CONFLICT parked, EXCLUSIVE rung 4 not written up (declared done by learner) | ✓ (oral, gap E-006) | | | next; open: describe2 `&String` question from l-03 |
| 05 | 05-structs-and-methods | | | | | | |
| 06 | 06-enums-pattern-matching | | | | | | |
| 07 | 07-collections-and-strings | | | | | | |
| 08 | 08-error-handling | | | | | | |
| 09 | 09-generics-and-bounds | | | | | | |
| 10 | 10-traits-intro | | | | | | |
| 11 | 11-standard-traits | | | | | | |
| 12 | 12-trait-objects-dispatch | | | | | | |
| 13 | 13-iterators | | | | | | |
| 14 | 14-modules-crates-visibility | | | | | | |
| 15 | 15-cargo-workspaces-architecture | | | | | | |
| 16 | 16-ecosystem-tour | | | | | | |
| 17 | 17-idiomatic-defensive-rust | | | | | | |

## Mini-project: text arena

Crate `crates/arena` (created on the first step). One step per lesson, each with a core part and an optional star part.

| lesson | step | core | star |
|--------|------|------|------|
