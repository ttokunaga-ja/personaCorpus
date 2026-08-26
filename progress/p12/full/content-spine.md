# p12 Full content spine — Support & Success

This is the frozen, synthetic facts contract for the p12 Full allocation. It
describes relationships which must remain consistent across formats; it is not
customer data, a production support record, or a Kio readiness claim.

## Synthetic organisation and people

- `org.heliodesk` — fictional support-platform organisation.
- `product.alpha` — synthetic customer workspace “Alpha Admin”.
- `product.beta` — synthetic customer workspace “Beta Sync”.
- `team.support-ops` — queue and escalation owners.
- `team.customer-success` — adoption and QBR reviewers.
- `team.engineering-liaison` — synthetic product escalation contact.

## Fixed support facts

- `case.cs-260713-184` — synthetic case family used for cross-format examples.
- `ticket.tkt-18427` — fictional priority-two ticket in that case family.
- `incident.inc-260713-04` — synthetic service review incident.
- `policy.sla-4h` — escalation acknowledgement target of four business hours.
- `policy.retention-18m` — illustrative eighteen-month support-record retention.
- `metric.first-response-94-2` — fictitious first-response attainment, 94.2%.
- `metric.resolution-91-8` — fictitious resolution attainment, 91.8%.
- `metric.adoption-78-6` — fictitious adoption measure, 78.6%.
- `decision.dec-260713-006` — synthetic approval to publish a known-issue note.

## Dates and relationships

- `date.2026-07-13` — initial synthetic case review date.
- `date.2026-07-15` — follow-up review date.
- `date.2026-07-20` — QBR and closure review date.
- `period.2026-q3` — synthetic reporting period.
- `rel.case-to-ticket` — `case.cs-260713-184` contains `ticket.tkt-18427`.
- `rel.ticket-to-incident` — `ticket.tkt-18427` is analysed against `incident.inc-260713-04`.
- `rel.sla-to-escalation` — escalation material applies `policy.sla-4h`.
- `rel.qbr-to-adoption` — QBR material reports `metric.adoption-78-6`.
- `rel.known-issue-to-decision` — published guidance cites `decision.dec-260713-006`.
- `rel.scan-source` — every scan PDF derives from a local synthetic PNG input.

## Scope relationship contract

The allocation uses this mapping as a required semantic minimum; format and
filename variation never changes the named subjects or their measures.

- Every `p12-primary-01` `support/ticket-exports` artifact and every
  `support/escalations` artifact describes `case.cs-260713-184` /
  `ticket.tkt-18427`, references both `policy.sla-4h` and
  `rel.sla-to-escalation`, and retains `rel.case-to-ticket`.
- `support/known-issues`, `knowledge-base/drafts`, and
  `knowledge-base/published` analyse `incident.inc-260713-04` for that ticket
  and publish the approved known-issue guidance using
  `decision.dec-260713-006` and its ticket-to-incident / known-issue-to-decision
  relationships.
- `customers/customer-alpha/qbr` and `customers/customer-beta/qbr` identify
  their respective `product.alpha` or `product.beta` workspace and report the
  Q3 adoption measure `metric.adoption-78-6` through `rel.qbr-to-adoption`.
- `customers/customer-alpha/case-history` and
  `customers/customer-beta/case-history` identify their respective product,
  the fixed case and ticket, the incident relationship, the four-business-hour
  SLA, eighteen-month retention policy, and first-response / resolution
  measures `metric.first-response-94-2` and `metric.resolution-91-8`.
- `incidents/support-links` joins the case, ticket, incident, SLA, known-issue
  decision, and both operational measures.  Supporting desktop, mail, export,
  attachment, macro, archive, and shared-reference scopes use the same fixed
  support record as appropriate and never invent a customer, case, measure,
  or approval.

`metric.unresolved-17` records the M1 queue anchor of seventeen unresolved
synthetic cases at `date.2026-08-04`; it is contextual only and does not alter
the fixed Full case, Q3 measure, or July review chronology above.

## Safety and production boundary

- `term.synthetic-only` — names, organisations, events, account references, and
  measures are invented for defensive corpus testing.
- `term.no-credentials` — do not create credentials, secrets, tokens, or private keys.
- `term.no-real-pii` — do not create real customer or personal information.
- `term.no-kio-claim` — physical artifacts do not claim indexing, search,
  history, chunk counts, performance, or Kio readiness.

Every Full artifact must use only relevant IDs from this spine, retain the
same values across linked text, Office, PDF, image, and scan materials, and
state synthetic context where a reader might otherwise infer a real record.
