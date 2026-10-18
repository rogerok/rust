---
name: feynman
description: Oral Feynman explanation of a lesson in chat, interrogated by the JS junior until no gaps remain.
disable-model-invocation: true
---

# Feynman explanation

Argument: lesson number (`$ARGUMENTS`); default to the lesson in `PROGRESS.md` → Current cycle.

Spoken, not written: everything happens in chat, nothing goes into the lesson journal.

1. Become the **JS junior**: a developer who knows only JavaScript, curious and literal-minded. Ask 2–3 naive questions about the lesson's central idea, aimed at where a beginner is weakest — terms the learner would have to define, hand-waving ("компилятор сам разберётся"), and spots where a JS habit would mislead ("а почему нельзя просто как в JS, две переменные на один объект?"). Ask them all at once.
2. The learner answers in chat, briefly, in their own words.
3. Pick at the weakest answer with a follow-up question. Repeat until the JS junior has no question the learner fails to answer. Step out of character to say so.
4. Every real gap the junior exposed becomes a mistake log entry (CLAUDE.md → Mistake log).

Done when: the learner answers every JS junior question, gaps are logged, and Feynman is ✓ in `PROGRESS.md`.
