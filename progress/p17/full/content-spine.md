# p17 Full content spine

- Persona: `p17` / Construction Project Manager
- Contract: `p17-content-spine-v1`
- State: frozen for `p17-full-master-v1`
- Content language: ja-JP and en-US are permitted; all timestamps are explicit JST or UTC
- Synthetic-use notice: every organization, project, drawing, role, schedule, identifier, image, and measurement below is fictional
- Safety boundary: no real address, site access information, worker/customer data, credentials, proprietary drawings, or private source material may appear in a derived artifact

This contract applies to every p17 scope. It preserves the accepted M1 anchors
**Harbor Build**, **Project Alpha 2026**, **2026-07-29**, **Change Order
CO-014**, and **JPY 18,600,000**. An artifact may include only relevant facts,
but may not contradict this contract.

## Scope-to-project boundary

The Full master assigns one project meaning to every artifact row. This is a
semantic boundary as well as a path boundary: a row must not introduce a
project that is absent from its `content_spine_ids`.

- `p17-primary-01` through `p17-primary-06` are Project Alpha-only scopes.
  They may cite `project.alpha-2026`, `change.co-014`, and
  `measure.co-014-jpy`; they never cite Project Beta.
- `p17-primary-07` through `p17-primary-10` are Project Beta-only scopes.
  They may cite `project.beta-2026`; they never cite Project Alpha, CO-014, or
  the CO-014 value.
- `p17-primary-11`, `p17-primary-12`, and every secondary scope are shared
  portfolio scopes. Their normal rows cite only `program.harbor-2026`, not
  either project. A future explicitly comparative row would have to name both
  project IDs and make the comparison explicit in its allocated content; the
  frozen v1 master intentionally contains no such rows.

## Stable project registry

| Spine ID | Frozen fact |
| --- | --- |
| `org.harbor-build` | Harbor Build is a fictional construction-management organization using only synthetic planning records. |
| `program.harbor-2026` | Harbor Build 2026 is a fictional portfolio containing bounded Project Alpha and Project Beta work packages. |
| `project.alpha-2026` | Project Alpha 2026 is a synthetic mixed-use retrofit project; its records use fictional zone names and no real site coordinates. |
| `project.beta-2026` | Project Beta 2026 is a synthetic interior-renewal project linked only by portfolio reporting, not by a shared physical site. |
| `system.fieldnote` | FieldNote is a fictional offline construction-notes application represented solely by exports and synthetic summaries. |
| `system.bim-handoff` | BIM Handoff is a fictional export register; artifacts may use synthetic model IDs but no executable or live model endpoint. |
| `team.project-controls` | Project Controls owns schedule, cost, change-order, and submittal coordination. |
| `team.site-review` | Site Review owns synthetic daily observations, RFI triage, and meeting follow-up records. |

Permitted role aliases are `controls@harbor.invalid`, `site-review@harbor.invalid`,
`design-review@harbor.invalid`, and `vendor-desk@harbor.invalid`. The `.invalid`
domain is deliberate and must not be replaced.

## Date, change, and measurement registry

| Spine ID | Frozen fact |
| --- | --- |
| `date.2026-07-29` | Primary coordination date. Morning review begins 09:00 JST; closeout review is recorded at 16:30 JST. |
| `date.2026-07-30` | RFI and submittal reconciliation date. |
| `date.2026-08-03` | Change-order approval and cost-control checkpoint. |
| `period.2026-q3` | Portfolio reporting period from 2026-07-01 through 2026-09-30. |
| `change.co-014` | CO-014 is the synthetic Project Alpha change order for a revised equipment-support coordination package. |
| `measure.co-014-jpy` | Approved synthetic CO-014 value is JPY 18,600,000; every related budget, workbook, report, and slide uses the same value. |
| `rfi.rfi-042` | RFI-042 asks for a bounded synthetic clarification on support-zone sequencing; it contains no actual drawing detail. |
| `submittal.sub-118` | SUB-118 is a fictional material-submittal review package with a synthetic disposition. |
| `decision.dec-260729-014` | DEC-260729-014 records acceptance of the CO-014 coordination plan subject to the RFI-042 closeout note. |
| `measure.schedule-buffer` | The synthetic schedule buffer is 4 working days after the 2026-08-03 checkpoint. |

## Cross-format consistency rules

- Project Alpha artifacts use `change.co-014`, `measure.co-014-jpy`, and the
  2026-07-29 to 2026-08-03 sequence when relevant.
- Project Beta artifacts use the same synthetic portfolio period but never cite
  Alpha-only CO-014 facts or claim that they share a physical address or live
  jobsite with Project Alpha.
- Only Project Alpha XLSX, DOCX, searchable PDF, PPTX, structured data, and
  image captions use CO-014 = JPY 18,600,000. Shared portfolio artifacts may
  use the checkpoint date but must not imply that the change order applies to
  Project Beta.
- Images and scan-source artwork are fictional diagrammatic, marked-up, or
  field-note-style visuals; they are never reproductions of real drawings.
- `pdf_scan` artifacts must have no text layer and must be derived from their
  documented final image dependency when one is present. If a scope has no
  unreserved final image source, its row specifies a scratch-only ImageGen PNG
  input that is never a final artifact.
- Network, contact, and source references remain synthetic. Use only `.invalid`
  email aliases and documentation address ranges when an address is necessary.

## Production and evaluation boundary

| Spine ID | Frozen fact |
| --- | --- |
| `term.synthetic-only` | The corpus is synthetic defensive evaluation material, not a real construction record set. |
| `term.no-kio-claim` | Physical files and QA do not establish Kio indexing, history, chunks, search quality, or performance. |
| `term.no-live-site-access` | No derived artifact grants or implies access to a real site, building system, or private construction repository. |
