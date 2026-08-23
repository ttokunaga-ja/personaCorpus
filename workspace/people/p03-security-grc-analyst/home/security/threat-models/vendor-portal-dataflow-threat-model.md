# Vendor Portal Dataflow Threat Model

## Dataflow

Vendor questionnaires enter a segregated portal, are screened by vendor-risk analysts, and are
summarized into a control assessment. Attachments are retained only when relevant to an approved
assessment purpose. The portal exchanges status metadata with the internal register, not full
attachment content.

## Key risks

| Dataflow stage | Concern | Mitigation |
|---|---|---|
| Invitation | wrong recipient receives a request | approved vendor contact registry |
| Upload | unsupported or excessive content | size/type allowlist and quarantine review |
| Analyst review | conclusion lacks evidence | required citation field |
| Register update | vendor status becomes stale | expiry-based reassessment task |

## Security requirements

Portal access uses a vendor-specific, time-limited invitation. Internal analysts receive only
the portfolio permissions necessary for assigned reviews. Exports redact contact details unless
the audit request has an approved need. The system maintains a ledger of invitations, uploads,
review actions, and disposition decisions.

## Assurance

Monthly sampling verifies that questionnaire answers, cited evidence, and risk ratings agree.
The vendor-risk owner reviews expired invitations and unresolved upload quarantines. Any data
classification discrepancy is handled as a privacy and records-management issue before the
assessment can close.
