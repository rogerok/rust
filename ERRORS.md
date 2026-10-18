# Журнал ошибок

<!--
Format (agent-maintained, content in Russian):

### E-NNN · <short title>
- lesson: NN
- error: <compiler error code / symptom>
- thought: <what the learner believed>
- actually: <what is true>
- interval: 1 | 3 | 7 | 14
- due: YYYY-MM-DD

Correct warm-up answer → next interval (1→3→7→14), due = today + interval.
Correct answer at interval 14 → move the entry to "Усвоено".
Wrong answer → interval 1, due = tomorrow.
-->

## Активные

### E-001 · Порядок drop в конце блока
- lesson: 03
- error: неверное предсказание порядка в журнале natural_order
- thought: «сначала в расте умирают старые, а потом новые»
- actually: переменные умирают LIFO: последняя созданная освобождается первой (обратный порядок объявления)
- interval: 1
- due: 2026-10-19

### E-002 · Затенение не освобождает значение
- lesson: 03
- error: неверное предсказание журнала fifo_order с тремя `let _fifo`
- thought: «при повторном let с тем же именем старое значение должно умереть»
- actually: затенение отнимает только имя; старое значение живёт до конца блока и умирает по LIFO вместе с остальными
- interval: 1
- due: 2026-10-19

### E-003 · mut не меняет тип
- lesson: 02
- error: E0308 mismatched types (разминка: `let mut x = "42"; x = x.len();`)
- thought: «с `mut` можно присвоить переменной значение другого типа»
- actually: «`mut` меняет только значение, тип остаётся прежним; сменить тип можно только через новую `let` (shadowing)»
- interval: 1
- due: 2026-10-19

## Усвоено
