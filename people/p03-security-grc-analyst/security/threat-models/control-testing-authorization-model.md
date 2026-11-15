# Control Testing Authorization Model

## Principle

Control testing is authorized only for a defined objective, approved evidence sources, and a
named testing window. This model prevents testing activity from becoming unbounded access.

## Authorization path

1. The control owner proposes objective, scope, and acceptable evidence.
2. The GRC lead checks independence and data-minimization requirements.
3. An approver grants time-bounded access using a specific role.
4. The tester records procedures and conclusion in the evidence register.
5. A reviewer checks that access ended and the conclusion is supported.

## Abuse cases and safeguards

| Abuse case | Safeguard |
|---|---|
| Tester reviews their own operational work | independence rule and conflict declaration |
| Broad access persists after test | automatic expiry and removal report |
| Unapproved system is included | scope-to-asset mapping review |
| Result is changed without trace | append-only review history |

## Assurance measure

For each quarter, sample completed tests and confirm approval, access expiry, source citations,
and reviewer independence. Escalate repeated deviations as a control-design issue rather than
an isolated paperwork deficiency.
