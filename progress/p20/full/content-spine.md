# p20 Full content spine

- Persona: `p20` / Investigative Journalist
- Contract: `p20-content-spine-v2`
- State: frozen for `p20-full-master-v2` after dedicated Alpha/Beta filename
  subject remediation; no Full artifact has been accepted under the discarded
  defective v1 allocation.
- All people, institutions, records, identifiers, images, correspondence, and
  measures are synthetic. No artifact represents a real source, contact,
  location, private record, credential, or unpublished material.

This contract preserves the accepted M1 anchors **Civic Ledger**, **Story Alpha
2026**, **2026-08-22**, **FOIA batch 17**, and verification date
**2026-08-19**. It applies to every Full addition without asserting any Kio
indexing, history, search, chunk, performance, or provenance result.

## Shared facts

| Spine ID | Frozen fact |
| --- | --- |
| `org.civic-ledger` | Civic Ledger is a fictional public-interest newsroom using wholly synthetic reporting material. |
| `project.story-alpha-2026` | Story Alpha 2026 is a synthetic records-accountability investigation with no real subject or source. |
| `project.story-beta-2026` | Story Beta 2026 is a separate synthetic follow-up package used only to model a journalist's working files. |
| `record.foia-batch-17` | FOIA batch 17 is a fictional, redacted-style source bundle; it contains no government, personal, or confidential record. |
| `date.2026-08-19` | Verification date: 2026-08-19. |
| `date.2026-08-20` | Synthetic source-card reconciliation date: 2026-08-20. |
| `date.2026-08-21` | Synthetic editorial review date: 2026-08-21. |
| `date.2026-08-22` | Story Alpha working publication date: 2026-08-22. |
| `measure.source-cards-36` | The synthetic reporting map uses 36 source cards, all fictional and non-contactable. |
| `measure.fact-check-24` | The fact-check register contains 24 synthetic claims with disposition and evidence-reference fields. |
| `measure.foia-pages-17` | Batch 17 contains 17 synthetic page images for corpus illustration only. |
| `term.source-protection` | Source protection means minimizing invented identifiers and never inventing a real person's identity or contact detail. |
| `term.verification` | Verification records confidence, contradiction, and next-review state; it never certifies real-world truth. |
| `term.redaction` | Redaction is a visual/documentary convention applied only to synthetic content. |
| `term.no-kio-claim` | Corpus production and QA establish no Kio indexing, history, chunks, search quality, or performance claim. |
| `rel.alpha-evidence` | Alpha sources, transcript notes, FOIA records, drafts, and fact checks retain the same batch, date, and synthetic claim IDs. |
| `rel.beta-followup` | Beta notes may refer to the same reporting conventions but never imply a shared real source. |
| `rel.scan-source` | Each scan PDF is derived from a same-scope final ImageGen PNG, checked at original size for legibility and pixel/source-to-scan match, and has no accidental PDF text layer. |
| `rel.media-sidecar` | Synthetic audio, transcript sidecars, and analysis rows share only fictional clip IDs and durations. |
| `rel.closed-record` | Archive records preserve synthetic review status and avoid any personal or confidential record. |

Permitted aliases use only `.invalid`, for example `editorial-desk@civic-ledger.invalid`
and `records-review@civic-ledger.invalid`. Do not replace them with deliverable domains.

## Scope map

| Scope | Subject focus | Relationship |
| --- | --- | --- |
| `p20-primary-01` | Story Alpha sources | `rel.alpha-evidence` |
| `p20-primary-02` | Story Alpha transcripts | `rel.alpha-evidence` |
| `p20-primary-03` | Story Alpha FOIA | `rel.alpha-evidence` |
| `p20-primary-04` | Story Alpha drafts | `rel.alpha-evidence` |
| `p20-primary-05` | Story Alpha fact checks | `rel.alpha-evidence` |
| `p20-primary-06` | Story Beta sources | `rel.beta-followup` |
| `p20-primary-07` | Story Beta transcripts | `rel.beta-followup` |
| `p20-primary-08` | Story Beta FOIA | `rel.beta-followup` |
| `p20-primary-09` | Story Beta drafts | `rel.beta-followup` |
| `p20-primary-10` | Data analysis | `rel.alpha-evidence` |
| `p20-primary-11` | Media transcript sidecars | `rel.media-sidecar` |
| `p20-primary-12` | Pitches and research | `rel.alpha-evidence` |
| `p20-secondary-01` | Desktop working notes | `rel.alpha-evidence` |
| `p20-secondary-02` | Reference documents | `rel.alpha-evidence` |
| `p20-secondary-03` | Inbox material | `rel.alpha-evidence` |
| `p20-secondary-04` | Export reviews | `rel.alpha-evidence` |
| `p20-secondary-05` | Personal cloud notes | `rel.beta-followup` |
| `p20-secondary-06` | Team-shared edits | `rel.beta-followup` |
| `p20-secondary-07` | Recent mail | `rel.alpha-evidence` |
| `p20-secondary-08` | Closed archive | `rel.closed-record` |

## Allocation rules

1. Each scope first creates scope-local structured/tabular/log foundation facts.
2. Every DOCX, XLSX, searchable PDF, PPTX, image, and scan PDF depends on a
   scope-local foundation row. Image artifacts use ImageGen.
3. Every scan PDF additionally depends on a same-scope final image artifact;
   no scratch-only source, cross-scope edge, same-batch edge, or cycle is allowed.
4. Visual artifacts must preserve the relevant frozen date, batch number,
   synthetic claim/clip IDs, and measures where those facts apply.
5. Markdown, mail/HTML, media, and safe binary records complete their scope
   after its foundation and visual prerequisites. No planned row is an accepted
   artifact until a production checkpoint is written.
6. Dedicated Story Alpha scopes (`p20-primary-01` through `p20-primary-05`)
   name only Story Alpha in planned filenames and subject references. Dedicated
   Story Beta scopes (`p20-primary-06` through `p20-primary-09`) do the same
   for Story Beta. Shared scopes may use neutral or explicitly comparative
   Alpha/Beta terminology only where their scope map calls for it.
