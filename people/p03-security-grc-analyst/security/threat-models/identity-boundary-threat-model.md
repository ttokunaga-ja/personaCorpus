# Identity Boundary Threat Model

## Objective

Protect identity assertions used by the GRC workspace while keeping access decisions auditable.
The model concerns federated sign-in, role mapping, and reviewer authorization; it excludes
credential collection and production configuration changes.

## Assets and boundaries

Identity provider claims cross into the workspace at sign-in. A role-mapping service converts
approved groups into least-privilege roles. A separate audit boundary records grants, removals,
and failed authorization attempts. Control owners may submit evidence, reviewers may decide,
and administrators may maintain mappings, but no single role may perform all three actions.

## Risk analysis

| Risk | Safeguard | Assurance activity |
|---|---|---|
| Stale group membership | time-bounded access review | quarterly recertification |
| Excessive default role | deny-by-default mapping | weekly entitlement sample |
| Session reuse after role change | short session lifetime | event correlation review |
| Service account ambiguity | named purpose and owner | monthly ownership report |

## Operating requirements

Role changes require a recorded business reason, approver, and effective date. The audit log
must preserve both the prior and new authorization result. Alerts for privilege elevation are
routed to the security queue, where an analyst validates context before escalation. Evidence
exports use role-filtered views so an auditor can inspect decisions without inheriting authoring
rights.

## Residual risk

External identity availability remains a dependency. The documented response is to pause
non-urgent approvals, maintain an offline decision queue, and reconcile queued decisions once
normal identity assurance returns.
