---
name: session-start
description: Open a session — warm-up from the mistake log, then a pre-drill on the upcoming lesson.
disable-model-invocation: true
---

# Session start

Argument: upcoming lesson number (`$ARGUMENTS`); default to the lesson in `PROGRESS.md` → Current cycle.

## 1. Warm-up

Once per sitting. Read `ERRORS.md` and `PROGRESS.md`.

1. Pick 3 questions: overdue and due-today entries in `ERRORS.md` first (oldest `due` first); fill remaining slots with questions on the most recent lesson whose warm-up criterion is still ✗, then on any earlier lesson. Turn each entry into a fresh question or a tiny prediction drill — test the idea, never repeat the entry's wording.
2. Ask all 3 at once. The learner answers from memory, no lookups.
3. Grade each answer as correct or wrong, explaining the wrong ones Socratically (a question first, the answer only if they stay stuck).
4. Apply the interval rules from the `ERRORS.md` header to every entry asked.
5. If the questions covered a lesson with a pending warm-up criterion, record ✓ (≥2 of 3 correct) or keep ✗ in `PROGRESS.md`; a ✗ keeps that lesson's material in future warm-ups.

Done when: 3 answers graded, `ERRORS.md` intervals updated, `last warm-up` in `PROGRESS.md` set to today.

## 2. Pre-drill

Once per lesson, before the learner reads it. Skip if `pre-drill done: yes`.

1. Fetch the lesson text. Write 2 prediction drills into `drills/NN-1.rs` and `drills/NN-2.rs`: standalone `fn main()` programs, ≤15 lines, each probing one central idea of the lesson. At least one should fail to compile.
2. For each drill, the learner predicts: compiles or not (and which line fails), what it prints, where drop fires (when relevant).
3. Reveal by running `rustc --edition 2024 drills/NN-k.rs -o /tmp/drill && /tmp/drill`. Show the real output or the compiler error.
4. Ask the learner to note prediction vs reality in the lesson journal's «Пре-дрилл» section. A wrong prediction here is expected: celebrate it as the thing the lesson will explain, and log nothing yet.

Done when: both drills predicted, revealed and noted; `pre-drill done: yes` in `PROGRESS.md`.
