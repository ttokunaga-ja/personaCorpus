# Error-budget policy exception

## Purpose and trigger
Use when a team requests a temporary release-policy exception while Checkout or Identity has consumed its normal error budget.

## Impact signals
Budget is exhausted or trending to exhaustion, yet a change is claimed to reduce risk, restore compliance, or address a time-bound operational requirement.

## Safe prerequisites
Require incident commander or reliability duty-manager review, a named change owner, rollback path, and quantified blast radius. An exception cannot bypass safety controls or required peer review.

## Diagnosis
1. Verify current budget and contributing incidents from the SLO dashboard.
2. Confirm the proposed change is remediation, not feature delivery disguised as remediation.
3. Evaluate test evidence, canary scope, dependency risk, and monitoring coverage.

## Mitigation
1. Record the exception in the change system with expiry no later than 24 hours.
2. Limit deployment to a 5% canary and define explicit abort thresholds.
3. Assign an observing on-call and hold at the canary for 20 minutes before any progression.

## Verification
The remedial metric improves, no abort threshold is crossed, and the exception record includes outcome and expiry closure.

## Rollback and escalation
Automatically halt at the defined threshold; roll back through the approved release controller. Escalate disagreements to the reliability duty manager, who may deny the exception.

## Evidence capture
Store budget chart, risk assessment, approvals, canary readings, decision time, and expiry confirmation.

## Owners and review date
Owner: Reliability Governance. Review: 2026-10-13.

## Cross-references
[Checkout error-budget breach](checkout-svc17-error-budget-breach.md), [Terraform plan drift response](terraform-plan-drift-response.md), [incident commander handoff](incident-commander-handoff.md).
