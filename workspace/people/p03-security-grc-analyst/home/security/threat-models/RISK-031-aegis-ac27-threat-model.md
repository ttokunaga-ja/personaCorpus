# RISK-031: Aegis Control AC-27 Threat Model

**System:** Cobalt Harbor Systems control-evidence service  
**Assessment date:** 2026-07-13  
**Evidence reference:** evidence set 184  
**Decision:** Treat RISK-031 as a monitored, medium residual-risk item.

## Scope and assets

This model covers the policy evidence submitted for Aegis Control AC-27: files, hashes,
review decisions, and the immutable audit trail. It excludes production administration and
does not authorize any testing against live systems. The critical assets are the evidence
object, its classification label, approval history, and the linkage between a control test
and its reviewer.

## Trust boundaries

1. A control owner supplies a bounded evidence package to the intake queue.
2. An analyst validates format and provenance before the package enters the evidence store.
3. A reviewer records an approval or a deficiency through a separate role.
4. The audit export service produces a read-only package with a manifest.

## Threats and controls

| Scenario | Impact | Preventive control | Detective signal | Owner |
|---|---|---|---|---|
| Unapproved replacement of a file | Incorrect attestation | content hash and write-once version | manifest mismatch | Evidence custodian |
| Reviewer self-approval | Biased conclusion | role separation | daily approval conflict report | GRC lead |
| Mislabelled retention class | Premature disposal | controlled taxonomy | monthly label sampling | Records owner |
| Export omission | Incomplete audit response | reconciled item count | export-to-register comparison | Audit coordinator |

## Treatment and acceptance

The service must reject missing provenance, preserve prior versions, and require a second
reviewer when an exception changes a control conclusion. Weekly reconciliation compares the
register, storage manifest, and export count. Any mismatch opens a corrective-action ticket
and pauses attestation closure. Residual risk is accepted only through the risk committee,
with an expiry date and a verification sample. This is a defensive assurance model; it
contains no operational exploitation guidance.

## Review evidence

The next review verifies hashes, role assignments, and exception expiry. Success means every
submitted item maps to one control, one accountable owner, and one retained decision record.
