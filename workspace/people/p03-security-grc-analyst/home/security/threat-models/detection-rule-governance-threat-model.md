# Detection Rule Governance Threat Model

## Context

Detection rules used for assurance monitoring must be understandable, approved, and measured.
This model addresses governance of rule intent and evidence, not security testing techniques.

## Lifecycle

Rules are proposed with a business/control objective, data-source dependency, owner, expected
signal, and review interval. A peer reviewer checks false-positive and coverage assumptions.
Changes are versioned, tested on synthetic or previously approved samples, and linked to the
control evidence they support.

| Risk | Governance safeguard |
|---|---|
| Rule silently disabled | availability and change alert |
| Logic no longer matches policy | scheduled control-owner recertification |
| Alert data exceeds purpose | data-minimization review |
| Analyst conclusion is unreproducible | saved rule version and query context |

## Assurance

The monitoring owner reports rule health, coverage assumptions, and unresolved gaps monthly.
Retired rules retain a decision record so historical evidence remains interpretable.
