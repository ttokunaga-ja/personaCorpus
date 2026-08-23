# Evidence Ingestion Trust Boundaries

## Purpose

Define how submitted control evidence becomes an auditable record. The pipeline accepts only
approved business artifacts and validates metadata before storage.

## Boundary map

| Boundary | Input | Required check | Output |
|---|---|---|---|
| Submission | owner package | owner authorization and classification | intake receipt |
| Validation | receipt plus files | file type, checksum, completeness | verified package |
| Review | verified package | independent reviewer assignment | decision record |
| Retention | decision record | retention label and legal hold status | immutable index |

## Threat considerations

The main failure modes are an untrusted source being treated as approved, a partial package
being marked complete, and metadata being changed after review. Intake therefore assigns a
package identifier, records the submitting role, and computes a manifest before any reviewer
sees content. Validation failures remain visible as rejected records; deletion is not used as
a substitute for traceability.

## Controls

Automated checks enforce approved extensions and declared sensitivity. Human checks confirm
that the evidence supports the stated control objective. The reviewer can request clarification
but cannot alter the submitted file. Retention changes require records-owner approval and are
logged with a reason. A daily reconciliation compares intake receipts to stored manifests.

## Test plan

Sample ten accepted and five rejected packages each month. Confirm the source, checksum,
classification, decision, and retention label are mutually consistent. Report systemic gaps to
the GRC steering meeting.
