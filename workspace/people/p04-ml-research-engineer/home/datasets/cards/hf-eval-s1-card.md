# HF-Eval-S1 v3.2: Baseline Wording Slice

## Role in the benchmark

S1 contains 1,600 synthetic records designed to establish a stable baseline
for intent routing. Prompts use complete sentences, one dominant request, and
minimal distracting context. The slice asks whether a model can recognize the
taxonomy before it is exposed to intentionally compressed or conflicting
language in S2.

## Construction profile

| field | value |
| --- | --- |
| record count | 1600 |
| label policy | one primary intent per record |
| wording | clear, everyday support requests |
| negative cues | limited to harmless contextual detail |
| provenance | wholly synthetic authoring |

S1 is balanced by design, but not perfectly uniform: its label counts are
documented in the class-distribution ledger. Scenario templates were varied at
the wording level so that repeated outcomes do not imply duplicate utterances.

## Reading results

An S1 score is useful for spotting basic taxonomy gaps. It is not a claim that
the model will behave equally well with terse, frustrated, or compound asks.
For that contrast, pair it with S2 rather than averaging away the distinction.

## Handling note

Keep split membership fixed during comparisons. If a wording defect is found,
record it in the version history and publish a successor dataset rather than
moving a record between S1 and S2.
