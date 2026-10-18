---
name: lesson
description: Full lesson cycle for one lesson — runs or resumes the whole session ritual.
disable-model-invocation: true
---

# Full lesson cycle

Argument: lesson number (`$ARGUMENTS`); default to the lesson in `PROGRESS.md` → Current cycle.

The cycle is resumable: `PROGRESS.md` → Current cycle holds the step to resume from. Each step below delegates to another skill's file — read that file and follow it in place. After every finished step, write the next step into Current cycle before doing anything else, so an interruption loses nothing.

## 0. Locate

Read `PROGRESS.md`. If Current cycle is for this lesson, resume at its step; otherwise set Current cycle to this lesson, step `warm-up`, `pre-drill done: no`. Tell the learner in one line where you are starting.

A new sitting always opens with the warm-up, even when resuming mid-cycle.

## Steps

1. **warm-up** — `.claude/skills/session-start/SKILL.md`, section 1. Once per sitting.
2. **pre-drill** — same file, section 2. Once per lesson.
3. **reading** — Give the lesson link. Ask the learner to read it, walk every widget scenario, and solve the on-site tasks, then say «прочитал». Wait. Then ask one question about the most surprising thing in the lesson, and connect it back to the pre-drill they got wrong, if any.
4. **homework** — `.claude/skills/hw/SKILL.md`.
5. **feynman** — `.claude/skills/feynman/SKILL.md`.
6. **review** — `.claude/skills/hw-review/SKILL.md`.
7. **quiz** — Ask whether the on-site quiz is passed; record ✓ in `PROGRESS.md` when it is.
8. **close** — `.claude/skills/session-end/SKILL.md`. The warm-up criterion is checked in the next sitting's warm-up.

The learner may stop at any point: run step 8 then, recording the exact step to resume from.

Done when: steps 1–8 are complete for this lesson and Current cycle points at the next lesson.
