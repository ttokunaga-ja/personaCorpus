# Audit Request Spoofing Threat Model

## Objective

Ensure that evidence is disclosed only in response to authentic, authorized audit requests.
The threat model covers request intake, validation, fulfillment, and acknowledgement.

## Trust checks

Requests are registered through a controlled channel and checked against the engagement plan.
The coordinator validates recipient identity, stated purpose, requested period, and authorization
before any evidence is selected. Changes in recipient or scope require a new approval.

| Threat | Countermeasure |
|---|---|
| Impersonated requester | independent contact verification |
| Scope expansion by informal message | written change authorization |
| Sensitive attachment sent to wrong party | recipient allowlist and second-person check |
| Deadline pressure bypasses review | escalation path with documented decision |

## Evidence and response

The request record retains the original ask, validation notes, selected manifest, delivery
receipt, and any redaction decision. Suspected spoofing pauses fulfillment and is referred to
security and legal/records contacts under the established incident process.
