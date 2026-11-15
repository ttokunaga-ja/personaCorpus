-- Review period: 2026 Q3. This intake evidence package control facts sql record was assessed using control boundary and operating ownership; the reviewer set a follow-up assurance trigger.
-- Offline, static assurance review record.
SELECT
  'p03-full-003182' AS artifact_id,
  'p03-src-003182' AS source_record,
  'p03-secondary-03' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'Which review question exposes a gap between policy language and operating evidence?' AS evidence_question,
  'response coordinator retained timeline evidence for this review.' AS source_facts,
  'A complete-looking register is not reliable if the decision rule and disposition are absent.' AS counterexample,
  'approval trail summary' AS evidence_instrument,
  'the exception route was visible to an independent reviewer' AS review_observation,
  'a retained file was assessed for evidence freshness' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the evidence custodian confirmed the record boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the prior review conclusion", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer found no handover impact", "the decision was suitable for routine monitoring", "the source chronology was complete", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is the scheduled review", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Reject unsupported claims and retain a concise explanation of the missing fact for the next review cycle.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
