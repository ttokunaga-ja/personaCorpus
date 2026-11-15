# HF-Eval Version History

## Release ledger

| version | status | scope | compatibility note |
| --- | --- | --- | --- |
| v3.0 | retired | initial synthetic routing set | superseded after taxonomy clarification |
| v3.1 | retired | revised prompt wording | not comparable where records changed |
| v3.2 | current for R184 | 2,400 records across S1 and S2 | frozen comparison population |

## v3.2 contract

Version 3.2 fixes the label taxonomy, split membership, and class distribution.
It contains 1,600 S1 records plus 800 S2 records. Published scores for the
R184 comparison refer to this contract; a correction must not be folded into
the existing version.

## Change management

A new version requires a written reason, row-impact count, updated distribution
ledger, and a note about metric comparability. Editorial documentation changes
that do not alter records may be recorded separately, but cannot claim to alter
the dataset. Retired versions remain interpretable only with their own released
ledgers.

## Why this matters

Keeping the history explicit prevents a benchmark improvement from being
mistaken for a model improvement. It also lets reviewers distinguish data drift
from changes in the evaluated system.
