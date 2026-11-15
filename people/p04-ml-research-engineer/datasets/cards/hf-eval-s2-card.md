# HF-Eval-S2 v3.2: Stress Wording Slice

## What S2 tests

S2 contributes 800 synthetic records whose wording makes routing less
mechanical. It includes shorthand, interrupted narratives, mixed timing cues,
and clauses that mention a secondary topic without changing the labeled
primary intent. The aim is controlled linguistic stress, not simulation of
individual people or production traffic.

## Slice characteristics

| characteristic | design choice |
| --- | --- |
| size | 800 records |
| ambiguity | bounded; one adjudicated primary label remains |
| phrasing | terse, compound, or context-dependent |
| challenge | separate dominant request from incidental detail |
| data origin | synthetic only |

S2 has the same five labels as S1 and preserves the full taxonomy. Its smaller
size means a single error moves a per-label rate more than it would in S1, so
reviewers should report counts alongside percentages.

## Appropriate use

Use S2 to inspect failure modes after establishing baseline behavior, especially
where `refund_request` language includes delivery history or where
`account_access` descriptions include device trouble. It should not be used to
derive a real-world incidence rate or to tune on examples that will later be
claimed as held-out.

## Review convention

When a record seems dual-intent, the adjudication note identifies the action a
user expects first. Disagreement is logged as a taxonomy question, not repaired
by retroactively changing the model prediction.
