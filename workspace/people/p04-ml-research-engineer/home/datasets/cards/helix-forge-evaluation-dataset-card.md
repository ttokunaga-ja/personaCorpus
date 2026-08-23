# Helix Forge Evaluation Dataset Card

## Purpose and scope

Helix Forge Lab assembled **HF-Eval-S1/S2 v3.2** as a synthetic, closed-world
evaluation set for routing-intent experiments. It contains 2,400 constructed
support-style records: S1 contributes 1,600 routine formulations and S2 adds
800 deliberately awkward paraphrases, incomplete requests, and competing
intent cues. The set is an evaluation instrument, not a representation of a
customer population and not training material for a deployed service.

The reference result was recorded on 2026-07-13 in run R184: Model Alpha M-14
reached macro F1 0.873 (precision 0.881; recall 0.865). Model Beta B-07 is the
fixed comparator at macro F1 0.846. These figures describe this synthetic
benchmark only.

## Composition

| split | version | records | construction emphasis |
| --- | --- | ---: | --- |
| S1 | v3.2 | 1600 | direct requests and ordinary wording |
| S2 | v3.2 | 800 | ambiguity, ellipsis, and multi-clause phrasing |
| Total | v3.2 | 2400 | fixed evaluation population |

The five single-label intents are `account_access`, `delivery_status`,
`plan_change`, `refund_request`, and `technical_help`. Every record has one
primary label, a split identifier, a synthetic scenario family, and an audit
state. No source records, identifiers, contacts, or live account details were
used.

## Interpretation boundary

Use aggregate and slice metrics to compare model behavior under the same
frozen inputs. Do not infer prevalence, fairness, user satisfaction, or
production readiness from this card. A reviewer should inspect errors before
changing a threshold, label definition, or release decision.

## Stewardship

The dataset is versioned as a whole. Corrections require a new version and an
updated distribution ledger; silent row replacement is disallowed. The related
[label taxonomy](hf-eval-label-taxonomy.md) and
[sampling protocol](hf-eval-sampling-protocol.md) define how the instrument is
read and maintained.
