---
name: battle
description: Battle session — a mixed task over the week's lessons, then an optional text-arena mini-project step.
disable-model-invocation: true
---

# Battle session

No new lesson. Material: lessons marked completed or in progress this week in `PROGRESS.md`.

1. **Warm-up** — `.claude/skills/session-start/SKILL.md`, section 1.
2. **Mixed task** (~30–40 min). Design one task that forces the week's ideas to meet (e.g. ownership + borrowing: decide per parameter whether it moves, borrows or borrows mutably). Put it in the newest lesson crate as `src/battle_<date>.rs` (a module) or `src/bin/battle_<date>.rs`, with red tests and stubs per CLAUDE.md. Guide in Socratic mode. Done when green and reviewed briefly (2 points max, `.claude/skills/hw-review/SKILL.md` rules).
3. **Mini-project step** (optional, ≤30 min — ask whether there is time).
   - The arena lives in `crates/arena` (create it on the first step as a bin crate). Theme: characters with an inventory fighting turn by turn; each lesson's idea maps onto it (ownership = who holds an item, borrowing = inspect without taking, structs, enums for actions, errors for invalid moves…).
   - Next step = the earliest completed lesson without a row in `PROGRESS.md` → Mini-project.
   - Split it into a **core** part using only material of that lesson and earlier, sized so the learner can solve it with no hints, and a **star** part that may stretch further.
   - Write red tests for the core; the star part gets a short description, tests optional. Add the row to `PROGRESS.md` and stay hands-off on the core: rung 1 of the hint ladder at most unless the learner asks to climb.
   - A skipped step is fine: record it as skipped and move on next time.
4. **Close** — `.claude/skills/session-end/SKILL.md`.
