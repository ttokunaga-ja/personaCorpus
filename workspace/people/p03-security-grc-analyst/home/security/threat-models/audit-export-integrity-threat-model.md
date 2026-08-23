# Audit Export Integrity Threat Model

## Goal

Produce defensible, complete audit exports without exposing material outside the approved
request. The export is a derived package, never the authoritative evidence store.

## Threat scenarios

| Scenario | Control response |
|---|---|
| Requested item is omitted | compare request register to export manifest |
| File is altered after selection | capture hashes at selection and delivery |
| Confidential item is overshared | request-scoped authorization and redaction review |
| Export cannot be reproduced | retain query parameters, version, and timestamp |

## Integrity design

An audit coordinator opens a request record defining scope, recipient, and due date. The export
service selects only approved records, creates a numbered manifest, and performs a two-person
quality check. Delivery uses an approved channel and records receipt. Any post-delivery change
creates a superseding package rather than silently replacing the original.

## Monitoring

The coordinator reviews manifest count variances and late deliveries weekly. Quarterly drills
recreate a prior export from its selection record to prove that provenance and retention remain
sufficient. Exceptions document rationale, approver, expiry, and compensating controls.
