# HelioDesk reply-macro approval checklist

Applies to the approved HelioDesk reply set for 2026-08-04. The operating response SLA is 4 hours. The catalog is the source of record for macro ID, version, owner role, status, and required placeholders.

Before sending any macro:

- Confirm the catalog entry is `approved` and the file name and stable ID match.
- Complete every required placeholder listed in `reply-macro-catalog.yaml`; do not add unapproved placeholders.
- Verify that `{{case_id}}` is the current case and that the greeting and timing are appropriate for the recipient.
- Remove every `Operational context — do not include in the sent reply` annotation before sending.
- Use account verification only through the case’s approved reply channel; never request a password, one-time code, recovery code, API key, access token, or full payment details.
- Use a case-specific secure upload link for attachments. Ask for only material needed to investigate, and ask the customer to redact unrelated personal or customer data.
- State the 4-hour response SLA accurately. Do not promise a fix, outcome, credit, or time beyond support policy.
- Keep the tone direct, empathetic, and specific to the case. Do not expose another customer’s information or an internal queue detail.
- Check that no sample text, unresolved placeholder, or internal annotation remains in the outgoing reply.

Approval record: Support & Success Lead owns the set; each catalog entry records its individual version.
