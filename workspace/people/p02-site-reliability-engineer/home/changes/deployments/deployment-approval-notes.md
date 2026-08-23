# Deployment approval notes — 2026-07-13

## CHG-260713-17: Checkout retry mitigation

The incident commander approved the emergency mitigation after confirming that the retry-amplification pattern was consuming the remaining Checkout SVC-17 error budget faster than the on-call policy allows. The configuration is reversible, isolated to the checkout deployment, and has a tested rollback path.

Approval conditions:

- deploy one replica at a time;
- keep payment-provider and synthetic-probe signals visible throughout the window;
- stop and roll back if availability falls below the documented threshold;
- add the final signal review to the incident record.

## CHG-260715-04: Identity session-cache hotfix

The reliability rotation approved the standard hotfix subject to clean pre-flight session-store replication and authentication probes. The planned change does not alter account policy, stored customer information, or access credentials. The prior deployment revision is the rollback target.
