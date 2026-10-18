---
name: session-end
description: Close a session — mistake log, progress, journal check and a proposed commit.
disable-model-invocation: true
---

# Session end

Callable at any moment, also mid-step.

1. **Mistake log.** List the misunderstandings that surfaced this session. For each one not yet in `ERRORS.md`, draft an entry, confirm `thought`/`actually` wording with the learner, set `interval: 1`, `due: tomorrow`.
2. **Progress.** Update `PROGRESS.md`: lesson table criteria, and Current cycle set to the exact step and homework to resume from. If all four completion criteria hold (or the learner declares it), mark the lesson completed.
3. **Recap.** In 3 lines of Russian: what was learned, what is next, the next due mistake log date.
4. **Commit.** Run `git status`, propose a message like `l-03: HW-RS-03-FIX-MOVE green, feynman done`, and commit only after the learner says yes. Pre-commit runs `just check`; a failure means fixing and retrying, not `--no-verify`.

Done when: `ERRORS.md` and `PROGRESS.md` reflect the session, and the commit is made or declined.
