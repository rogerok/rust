# Learning Rust with an AI mentor

A system for working through the BatSchool "Rust" course (section 23) together with an AI mentor. Near-term milestone: lessons 1–17; the goal is understanding, not speed. Terms are canonical in English; the Russian name in parentheses is the word the mentor uses when talking to the learner.

## Language

### Course

**Lesson** (Урок):
One numbered lesson of the BatSchool course (`03`, `04`, …), consisting of theory, a widget, on-site tasks, a quiz and homework.
_Avoid_: lecture, chapter, topic, session

**Homework** (ДЗ):
An assignment from a lesson, identified by its course ID (`HW-RS-03-FIX-MOVE`); the backbone of practice.
_Avoid_: task, exercise

### Practice

**Red test** (Красный тест):
A test the mentor writes for a homework before it is solved; the homework is done when all its red tests pass.
_Avoid_: check, autotest

**Stub** (Заглушка):
A `todo!()` body with the target signature, shipped alongside red tests so they compile; withheld when designing the signature is the point of the homework.
_Avoid_: skeleton, template

**Prediction drill** (Дрилл предсказания):
A short code fragment for which the learner predicts, before running it, whether it compiles, what it prints and where drop fires.
_Avoid_: quiz, puzzle

**Twin task** (Задача-близнец):
A small task, similar to one the learner just received a full solution for, which the learner then solves alone.
_Avoid_: extra task, follow-up exercise

**Battle session** (Боевая сессия):
A session with no new lesson in which the mentor sets a mixed task covering the week's material.
_Avoid_: test, exam

**Mini-project** (Мини-проект):
An optional text-arena project that grows by one small step per lesson, each step split into a core part meant to be solved alone and an optional star part; skipping a step breaks nothing.
_Avoid_: pet project, capstone

### Rhythm

**Session** (Сессия):
One sitting with the mentor, following the ritual: warm-up → pre-drill → lesson → homework → Feynman explanation → review → close.
_Avoid_: lesson, call

**Warm-up** (Разминка):
The opening of a session: a few questions on past material, answered from memory, drawn from the mistake log.
_Avoid_: revision, quiz

**Pre-drill** (Пре-дрилл):
Prediction drills on the lesson's topic solved before reading it; mistakes here are expected and useful.
_Avoid_: entrance test

**Full lesson cycle** (Полный цикл урока):
The whole session ritual for one lesson, started by a single command and resumable from where it stopped.
_Avoid_: run, pipeline

**Completed lesson** (Пройденный урок):
A lesson whose completion criteria are met, or which the learner has declared completed.
_Avoid_: closed, done

### Mentor role

**Socratic mode** (Сократический режим):
The mentor's default mode: questions and hints instead of solution code.
_Avoid_: help, hinting

**Hint ladder** (Лестница подсказок):
Four levels of help: guiding question → where to look → code fragment → full solution. The mentor climbs a level only on the learner's command.
_Avoid_: hints, help

**Feynman explanation** (Объяснение по Фейнману):
The learner's plain-language retelling of the lesson, written in the lesson journal and then interrogated by the JS junior until no gaps remain.
_Avoid_: summary, notes

**JS junior** (JS-джун):
The persona the mentor plays during a Feynman explanation: knows only JS and asks naive questions.
_Avoid_: student, interlocutor

**False analogy** (Аналогия-ловушка):
A JS/TS analogy that helps understand Rust but breaks somewhere; the mentor must name where.
_Avoid_: comparison, example

**Review** (Ревью):
The mentor's pass over code after its red tests pass: idiomaticity and "why this way and not another" questions.
_Avoid_: pre-test code review

### Reflection

**Lesson journal** (Журнал урока):
The learner's notes for one lesson, including the Feynman explanation.
_Avoid_: notes, summary

**Mistake log** (Журнал ошибок):
A running list of misunderstandings (what the error was, what I thought, what turned out to be true), each with a spaced-repetition interval; the source for warm-ups and new drills.
_Avoid_: log, bug report
