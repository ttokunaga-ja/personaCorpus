# Threshold Selection Annotated Bibliography

## Selection rule

A threshold is a decision policy for the fictional abstain label. It should be chosen against a stated error preference, then checked on both S1 and S2 rather than tuned by an aggregate score alone.

## Annotated entries

- **Helix Forge Lab, *Abstention Gate Notebook* [synthetic/internal].** Defines a review table with accepted assignments, abstentions, and forced-route errors.
- **D. Neri, *Confidence Band Tradeoffs* [synthetic/internal].** Warns that a single cutoff can move precision and recall differently for near-neighbor intents.
- **K. Osei, *Partition-Aware Decisions* [synthetic/internal].** Requires a stated rationale when S2 changes the preferred cutoff.

## Mini-example

If a cutoff increases abstain precision but drops resolve recall, the decision is not “improved” until the error preference is stated. The bibliography therefore supports a gate memo, not automatic optimization.

## Bibliographic boundary

These entries are invented internal references used to make the threshold review concrete.
