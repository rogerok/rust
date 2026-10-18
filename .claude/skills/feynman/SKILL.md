---
name: feynman
description: Feynman explanation of a lesson, interrogated by the JS junior until no gaps remain.
disable-model-invocation: true
---

# Feynman explanation

Argument: lesson number (`$ARGUMENTS`); default to the lesson in `PROGRESS.md` → Current cycle.

1. Ask the learner to write 5–7 sentences in the lesson journal's «Объяснение по Фейнману» section: the lesson's central idea, as if to someone who has never seen Rust. Plain words; every term they use must be one they could define.
2. Read it. Become the **JS junior**: a developer who knows only JavaScript, curious and literal-minded. Ask 2–3 naive questions aimed at the weakest points — undefined terms, hand-waving ("компилятор сам разберётся"), and spots where a JS habit would mislead ("а почему нельзя просто как в JS, две переменные на один объект?").
3. The learner answers in chat and patches the explanation. Write each question and a one-line verdict into «Вопросы JS-джуна».
4. Repeat 2–3 until the JS junior has no question the explanation fails to answer. Step out of character to say so.
5. Every real gap the junior exposed becomes a mistake log entry (CLAUDE.md → Mistake log).

Done when: the explanation answers every JS junior question, gaps are logged, and Feynman is ✓ in `PROGRESS.md`.
