# p19 Full content spine

- Persona: `p19` / Educator and Instructional Designer
- Contract: `p19-content-spine-v1`
- State: frozen for `p19-full-master-v1`
- Synthetic-use boundary: all people, learning records, courses, content,
  addresses, identifiers, score values, and media are fictional.

This contract extends, but never changes, the accepted M1 corpus. It is an
education-only corpus: it contains no real learner record, credential, private
source material, live LMS access, medical advice, or operational security data.

## Stable learning environment

| Spine ID | Frozen fact |
| --- | --- |
| `org.maple-learning` | Maple Learning is the fictional learning organization represented by this persona. |
| `standard.ml-19` | Maple Learning Design Standard ML-19 requires a stated learning objective, accessible materials, a transparent rubric, and revision-oriented feedback. |
| `date.2026-09-14` | Anchor snapshot date for this synthetic instructional environment. |
| `course.alpha-2026-t1` | Course Alpha 2026 Term 1 develops evidence notes, source comparison, claim-evidence-reasoning, peer feedback, revision, and capstone synthesis. |
| `course.beta-2026-t2` | Course Beta 2026 Term 2 develops inquiry questions, scenario mapping, model testing, design critique, and learning transfer. |
| `cohort.ml26` | The synthetic cohort contains exactly 36 learner IDs, `ML26-S001` through `ML26-S036`; these are identifiers only, not persons. |
| `rubric.common-20` | Each shared assignment uses Evidence 6 + Reasoning 6 + Organization 4 + Format 4 = 20 points. |
| `formula.score-total` | A score total is `E + R + O + F`, bounded 0 through 20; aggregate rate is `total / 20`. |
| `dataset.lms-216` | The synthetic LMS exports contain 216 score rows: 36 learner IDs × 6 assignments. |
| `term.no-kio-claim` | Corpus production and QA make no Kio indexing, history, chunking, search-quality, or performance claim. |

## Assignments and learning relationships

| Spine ID | Frozen fact |
| --- | --- |
| `assignment.a101` | A-101 Evidence Map belongs to Course Alpha. |
| `assignment.a102` | A-102 Source Comparison belongs to Course Alpha. |
| `assignment.a103` | A-103 Claim-Evidence-Reasoning Brief belongs to Course Alpha. |
| `assignment.b201` | B-201 Scenario Map belongs to Course Beta. |
| `assignment.b202` | B-202 Model Test belongs to Course Beta. |
| `assignment.b203` | B-203 Design Critique belongs to Course Beta. |
| `objective.evidence-literacy` | Learners identify relevant evidence, distinguish claims from support, and state an evidence-based rationale. |
| `objective.modeling` | Learners represent an inquiry or scenario, test an explanatory model, and revise it with observed feedback. |
| `objective.design-critique` | Learners use criteria and evidence to critique a design and propose a practical revision. |
| `objective.accessibility` | Materials support multiple entry points, clear structure, readable alternatives, and transparent assessment criteria. |

## Scope relationship registry

| Spine ID | Required relationship |
| --- | --- |
| `rel.alpha-learning-sequence` | Course Alpha lessons, readings, assignments, synthetic work, and grade exports retain the Alpha objectives and A-101/A-102/A-103 order. |
| `rel.beta-learning-sequence` | Course Beta lessons, readings, assignments, synthetic work, and grade exports retain the Beta objectives and B-201/B-202/B-203 order. |
| `rel.rubric-and-score` | Item bank, rubric, LMS exports, feedback, workbooks, PDFs, decks, and learner work retain the same 20-point component formula. |
| `rel.synthetic-cohort` | Any roster, work sample, score row, or aggregate names only the ML26 synthetic ID range and never invents personal identity data. |
| `rel.accessible-materials` | Reading, lesson, presentation, and reference artifacts provide accessible alternatives, clear headings, and assessment language appropriate to their medium. |
| `rel.lms-export` | The LMS export set reconciles six assignments and 216 synthetic score rows with course and rubric facts. |
| `rel.scan.source` | Every scan PDF depends on a final ImageGen image artifact in the same scope; scans are image-only and retain no accidental text layer. |
| `rel.professional-learning` | Professional-development and personal teaching reflections use the same objectives, rubric, and accessibility principles without adding learner records. |
| `rel.archive.learning-record` | Closed/archive materials preserve fictional course references and synthetic identifiers only. |

## Scope reuse map

| Scope | Primary IDs | Relationship |
| --- | --- | --- |
| `p19-primary-01` | `course.alpha-2026-t1`, `objective.evidence-literacy`, `assignment.a101` | `rel.alpha-learning-sequence` |
| `p19-primary-02` | `course.alpha-2026-t1`, `objective.accessibility`, `assignment.a102` | `rel.alpha-learning-sequence` |
| `p19-primary-03` | `course.alpha-2026-t1`, `assignment.a101`, `assignment.a103` | `rel.alpha-learning-sequence` |
| `p19-primary-04` | `cohort.ml26`, `rubric.common-20`, `assignment.a102` | `rel.synthetic-cohort` |
| `p19-primary-05` | `course.beta-2026-t2`, `objective.modeling`, `assignment.b201` | `rel.beta-learning-sequence` |
| `p19-primary-06` | `course.beta-2026-t2`, `objective.accessibility`, `assignment.b202` | `rel.beta-learning-sequence` |
| `p19-primary-07` | `course.beta-2026-t2`, `assignment.b201`, `assignment.b203` | `rel.beta-learning-sequence` |
| `p19-primary-08` | `cohort.ml26`, `rubric.common-20`, `assignment.b203` | `rel.synthetic-cohort` |
| `p19-primary-09` | `rubric.common-20`, `formula.score-total`, `objective.evidence-literacy` | `rel.rubric-and-score` |
| `p19-primary-10` | `dataset.lms-216`, `formula.score-total`, `cohort.ml26` | `rel.lms-export` |
| `p19-primary-11` | `course.alpha-2026-t1`, `course.beta-2026-t2`, `objective.accessibility` | `rel.accessible-materials` |
| `p19-primary-12` | `standard.ml-19`, `objective.design-critique`, `objective.accessibility` | `rel.professional-learning` |
| `p19-secondary-01` | `course.alpha-2026-t1`, `course.beta-2026-t2`, `date.2026-09-14` | `rel.rubric-and-score` |
| `p19-secondary-02` | `standard.ml-19`, `objective.accessibility`, `rubric.common-20` | `rel.accessible-materials` |
| `p19-secondary-03` | `course.alpha-2026-t1`, `assignment.a101`, `course.beta-2026-t2` | `rel.accessible-materials` |
| `p19-secondary-04` | `dataset.lms-216`, `formula.score-total`, `rubric.common-20` | `rel.lms-export` |
| `p19-secondary-05` | `standard.ml-19`, `objective.design-critique`, `date.2026-09-14` | `rel.professional-learning` |
| `p19-secondary-06` | `course.alpha-2026-t1`, `course.beta-2026-t2`, `objective.evidence-literacy` | `rel.accessible-materials` |
| `p19-secondary-07` | `course.beta-2026-t2`, `assignment.b203`, `date.2026-09-14` | `rel.beta-learning-sequence` |
| `p19-secondary-08` | `course.alpha-2026-t1`, `course.beta-2026-t2`, `cohort.ml26` | `rel.archive.learning-record` |

## Allocation and dependency contract

1. Foundation rows are structured, tabular, code, log, and domain-binary
   sources. They precede every visual or Office dependency in the same scope.
2. Every spreadsheet, document, searchable PDF, and presentation depends on
   scope-local structured or tabular facts.
3. Every image uses ImageGen and depends on local foundation facts. Every scan
   PDF has a final same-scope ImageGen image dependency and has no text layer.
4. Dependencies are scope-local, strictly earlier batches, and acyclic.
5. Every row is allocation evidence only; `term.no-kio-claim` applies to all
   planned rows and no row represents an already-created Full artifact.
