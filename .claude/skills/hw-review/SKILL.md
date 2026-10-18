---
name: hw-review
description: Review green homework code for idiomatic Rust through "why this way" questions.
disable-model-invocation: true
---

# Review

Argument: lesson crate or homework ID (`$ARGUMENTS`); default to the lesson in `PROGRESS.md` → Current cycle.

1. Gate: run `cargo test -p <crate>` and `just check`. Anything red → send the learner back to the homework; review only green code.
2. Collect candidates: read the code, and run `cargo clippy -p <crate> --all-targets -- -W clippy::pedantic` as an extra lens.
3. Pick the 3–5 points that teach the most about the current or previous lessons: needless clones and moves, ownership the signature hides, idioms the lesson just introduced. Ignore style trivia rustfmt already settles; leave material from future lessons for a one-line teaser at most.
4. Present each point as a question ("почему здесь `clone`, а не …?", "что изменится, если …?"). Let the learner answer and make any change themselves; then re-run the gate.
5. A point the learner could not answer and that reveals a misunderstanding → mistake log entry.

Done when: every chosen point is discussed and either applied or consciously declined, and tests are still green.
