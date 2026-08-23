# Managed Endpoint GRC Workflow Threat Model

## Scope

Endpoint posture reports support assurance of managed devices. The workflow ingests inventory
summaries, evaluates policy compliance, and records remediation evidence without retaining
unnecessary user activity data.

## Threats

Unmanaged devices can be represented as compliant, report freshness can be mistaken for current
status, or a remediation closure can lack independent verification. To reduce these risks, the
inventory is reconciled to the approved asset registry, posture reports include a collection
timestamp, and closure requires a fresh evidence sample.

## Control design

| Workflow step | Required evidence | Accountable role |
|---|---|---|
| Inventory reconcile | count variance explanation | Endpoint owner |
| Policy review | approved baseline version | Security analyst |
| Remediation tracking | target date and owner | Service manager |
| Closure validation | post-change posture sample | Independent reviewer |

## Residual-risk management

Devices unable to meet the baseline require a documented exception with compensating controls,
business sponsor, and expiry. The GRC register reports exceptions separately from remediation
progress so decision makers do not mistake accepted risk for resolved risk.
