# Terraform plan drift response

## Purpose and trigger
Use when the scheduled infrastructure plan reports unmanaged drift in production Northstar resources.

## Impact signals
Plan changes an existing route, alert, capacity limit, or policy unexpectedly. Drift is not automatically an incident; classify service impact first.

## Safe prerequisites
Freeze applies for the affected workspace, identify the approved state backend, and obtain Infrastructure owner acknowledgement. Never run an unrestricted apply to discover intent.

## Diagnosis
1. Execute read-only `terraform plan -refresh-only -var-file=env/production.synthetic.tfvars` from the approved automation workspace.
2. Compare the plan with the last approved change record and controller event history.
3. Classify each difference as expected controller action, stale state, unauthorized change, or unknown.

## Mitigation
1. For expected controller changes, document and reconcile through the owned module.
2. For an unauthorized high-impact change, preserve evidence and restore only via a reviewed targeted plan.
3. For stale state, request platform state repair; do not edit state manually during incident response.

## Verification
The subsequent refresh-only plan has no unexplained changes and service health is unchanged.

## Rollback and escalation
Roll back a reviewed targeted change through the same automation pipeline. Escalate unknown network, identity, or alert-routing drift to Security and Infrastructure leadership.

## Evidence capture
Keep plan summary, change record, resource addresses, classification, and approval references; exclude state values that may be sensitive.

## Owners and review date
Owner: Infrastructure Automation. Review: 2026-10-12.

## Cross-references
[Error-budget policy exception](error-budget-policy-exception.md), [regional disaster recovery](regional-disaster-recovery.md), [incident commander handoff](incident-commander-handoff.md).
