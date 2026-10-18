---
name: hw
description: Set up a lesson's homework with red tests and stubs, then guide the learner through it in Socratic mode.
disable-model-invocation: true
---

# Homework

Argument: lesson number or homework ID (`$ARGUMENTS`); default to the lesson in `PROGRESS.md` → Current cycle.

## 1. Prepare

1. Fetch the lesson page and copy each homework's exact wording (ID, stars, text).
2. Ensure the lesson crate exists: `crates/l-NN-<short-name>` as a lib crate (`just new-lib l-NN-<short-name>`, then empty out the generated `lib.rs`), with a lesson journal `JOURNAL.md` copied in structure from an earlier lesson's journal: sections for pre-drill, each homework's questions, Feynman explanation, JS junior's questions.
3. Classify each homework:
   - **checkable** → red tests + stubs, following CLAUDE.md → Red tests and stubs;
   - **prediction** (predict-then-run, widget walkthroughs) → a journal section, no tests;
   - **on-site** → nothing in the repo; the learner pastes it if stuck.
4. Verify: `cargo test -p <crate> --no-fail-fast` shows every new test failing at runtime, `just check` passes.

Skip any homework that already has tests or a journal section.

## 2. Guide

Take the homework in order of stars, one at a time.

1. Restate the task in Russian in 2–3 sentences, name the test file and how to run it (`just test-lesson <crate>`), and stop. The learner works.
2. Respond in Socratic mode (CLAUDE.md). For prediction homework, check the prediction is written in the journal before they run anything.
3. When tests are green and `just check` is clean, mark the homework ✓ in `PROGRESS.md` and set Current cycle → `homework` to the next one.

Done when: every homework of the lesson is ✓ in `PROGRESS.md`, or the learner parks one explicitly (record it as parked).
