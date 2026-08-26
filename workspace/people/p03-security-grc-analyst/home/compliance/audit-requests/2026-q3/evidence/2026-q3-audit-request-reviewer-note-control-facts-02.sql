-- Review period: 2026 Q3. This audit request reviewer note control facts sql record was assessed using risk treatment and residual uncertainty; the reviewer retained a challenge response.
-- Offline, static assurance review record.
SELECT
  'p03-full-003318' AS artifact_id,
  'p03-src-003318' AS source_record,
  'p03-primary-07' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What minimal trace proves that the control remains attributable to its owner?' AS evidence_question,
  'risk owner retained decision record for this review.' AS source_facts,
  'A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence.' AS counterexample,
  'issue disposition register' AS evidence_instrument,
  'the source record separated assertion from corroboration' AS review_observation,
  'a change record was tested for reviewer independence' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the evidence custodian confirmed the record boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for routine monitoring", "the source chronology was complete", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is the scheduled review", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
