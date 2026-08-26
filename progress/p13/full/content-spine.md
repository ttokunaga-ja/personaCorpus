# p13 Full content spine — Aster Legal

This is the synthetic, token-free factual contract for p13 Full artifacts. It
extends the accepted M1 environment without changing an accepted M1 byte. All
organisations, matters, correspondence, people, documents, evidence and
metrics below are invented test data; they are not legal advice, privileged
material, personal data, credentials, or records of a real investigation.

## Shared facts

- `org.aster-legal` — Aster Legal, a fictional corporate privacy practice.
- `matter.alpha` — Matter Alpha, a synthetic preservation and privacy review.
- `matter.beta` — Matter Beta, a synthetic contract and guidance comparison.
- `control.ac-27` — Aegis Control AC-27, a fictional control-reference label.
- `date.2026-06-30` — synthetic record-date and reporting cut-off.
- `date.2026-07-12` — synthetic exception-review deadline.
- `measure.hold-42` — 42 synthetic preservation objects; never real custodians.
- `measure.evidence-184` — synthetic evidence-set count of 184.
- `risk.risk-031` — synthetic review issue RISK-031.
- `term.synthetic-legal-record` — a safe test record, never legal advice.
- `term.redacted-example` — an invented non-identifying illustration only.
- `term.privacy-by-design` — fictional governance terminology for this corpus.
- `rel.alpha-preservation` — Alpha notices, working records, evidence and board
  reporting must agree on the 42-object synthetic hold and 2026-07-12 deadline.
- `rel.beta-contract-review` — Beta correspondence, contract and due-diligence
  material uses the same synthetic Aster Legal facts and review vocabulary.
- `rel.control-evidence` — neutral board reports keep AC-27 and the synthetic
  evidence count consistent; they do not carry an Alpha or Beta matter anchor.
- `rel.synthetic-archive` — desktop, downloads, cloud, mail and archive files
  are fictional copies/references, not real correspondence.
- `rel.scan-source` — every scan PDF is derived from a same-scope synthetic
  ImageGen image or a deterministic scratch-only source declared by its row;
  its PDF has no text layer.
- `term.no-kio-claim` — artifacts make no Kio index, history, chunk, search,
  performance or readiness claim.

## Scope relationships

| Scope family | Contract IDs | Explicitly prohibited IDs |
| --- | --- |
| Matter Alpha correspondence / working (`primary-01`, `primary-02`) | `matter.alpha`, `rel.alpha-preservation`, `measure.hold-42` | `matter.beta`, `rel.beta-contract-review`, `risk.risk-031` |
| Matter Beta correspondence / working (`primary-03`, `primary-04`) | `matter.beta`, `rel.beta-contract-review`, `risk.risk-031` | `matter.alpha`, `rel.alpha-preservation`, `measure.hold-42` |
| Beta-local contracts, guidance, privacy and due diligence (`primary-05`–`primary-10`) | `matter.beta`, `rel.beta-contract-review`, `risk.risk-031`, `control.ac-27` | `matter.alpha`, `rel.alpha-preservation`, `measure.hold-42` |
| Alpha-local hold notices (`primary-11`) | `matter.alpha`, `rel.alpha-preservation`, `measure.hold-42` | `matter.beta`, `rel.beta-contract-review`, `risk.risk-031` |
| Neutral board reports (`primary-12`) | `control.ac-27`, `measure.evidence-184`, `rel.control-evidence` | `matter.alpha`, `rel.alpha-preservation`, `measure.hold-42`, `matter.beta`, `rel.beta-contract-review`, `risk.risk-031` |
| Desktop, reference, downloads, cloud, mail and archive (all secondary scopes) | `rel.synthetic-archive`, `date.2026-06-30`, `term.redacted-example` | `matter.alpha`, `rel.alpha-preservation`, `measure.hold-42`, `matter.beta`, `rel.beta-contract-review`, `risk.risk-031` |

## Cross-format rules

Structured/CSV facts are the source of truth for dependent XLSX, DOCX,
searchable PDF and PPTX artifacts within the same scope. ImageGen source images
precede their scan PDFs. `measure.hold-42` is permitted only in the Alpha and
Alpha-local hold scopes in the table; `risk.risk-031` is permitted only in the
Beta and Beta-local contract/privacy scopes. The neutral and secondary scopes
must not infer either matter anchor. All artifacts retain `term.no-kio-claim`.
