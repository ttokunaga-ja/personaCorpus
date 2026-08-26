# p16 Full content spine

- Persona: `p16` / Clinical Researcher
- Contract: `p16-content-spine-v1`; state: frozen for Full allocation.
- Every fact is synthetic. No patient, clinician, facility, credential, private study record, or medical advice is represented.

## Frozen facts

| ID | Fact |
| --- | --- |
| `org.sakura-clinical` | Sakura Clinical is a fictional research organization using a synthetic Windows clinical VDI. |
| `study.alpha-2026` | Study Alpha 2026 is a fictional synthetic-data protocol. |
| `study.beta-2026` | Study Beta 2026 is a fictional synthetic-data protocol. |
| `date.2026-09-02` | Review and reporting reference date. |
| `cohort.synthetic-48` | The synthetic cohort contains 48 non-identifying records. |
| `threshold.2-5mg` | The fictional review threshold is 2.5 mg; it is not a treatment recommendation. |
| `term.synthetic-only` | All derived files are synthetic defensive research-administration artifacts. |
| `term.no-kio-claim` | No derived artifact claims Kio indexing, history, search, chunks, or performance. |

All artifacts must use only the applicable facts above. The Full master assigns
them by scope as follows: `study.alpha-2026` is limited to primary-01 through
primary-03; `study.beta-2026` is limited to primary-04 through primary-06;
all shared scopes are study-neutral. `cohort.synthetic-48` appears only in the
synthetic-case, results, and statistics scopes (primary-02, -03, -05, -06,
and -11). `threshold.2-5mg` appears only in fictional protocol, guideline,
regulatory, safety-review, and statistics administration scopes (primary-01,
-04, -07, -09, -10, and -11); it never communicates treatment guidance.

Structured data precedes Office/PDF workbooks and reports. Images precede scan
PDFs within the same scope; where a scope has no remaining final image source,
an ImageGen PNG is created only under the row's declared `scratch_input` path
and never becomes a final artifact.
