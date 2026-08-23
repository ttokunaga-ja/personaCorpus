# Quarterly Attestation Threat Model

## Attestation purpose

Quarterly attestation asks control owners to state whether evidence and operating conditions
remain consistent with the approved control design. It is a governed assertion supported by
evidence, not a replacement for independent testing.

## Threats

| Threat | Mitigation |
|---|---|
| Owner attests without current evidence | evidence-date validation |
| Delegated response lacks authority | approved delegate registry |
| Non-response hidden by reporting | overdue escalation dashboard |
| Negative result rewritten as positive | immutable response history |

## Workflow

The cycle opens with a control inventory snapshot and closes only after responses, evidence
references, and exceptions are reconciled. The GRC team challenges inconsistent answers and
routes unresolved gaps to the risk register. Final reporting distinguishes attested, independently
tested, excepted, and overdue controls so leaders understand confidence levels.

## Metrics

Track on-time completion, evidence freshness, challenged responses, and unresolved exceptions.
Review trends quarterly to improve the attestation design and reduce reliance on manual follow-up.
