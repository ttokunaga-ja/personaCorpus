-- Review period: 2026 Q3. This intake request attachment control facts sql record was assessed using closure evidence and accountable disposition; the reviewer set a follow-up assurance trigger.
-- Offline, static assurance review record.
SELECT
  'p03-full-007416' AS artifact_id,
  'p03-src-007416' AS source_record,
  'p03-secondary-03' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'Which review question exposes a gap between policy language and operating evidence?' AS evidence_question,
  'named control owner retained approval boundary for this review.' AS source_facts,
  'A complete-looking register is not reliable if the decision rule and disposition are absent.' AS counterexample,
  'outcome verification note' AS evidence_instrument,
  'the approval boundary matched the operating record' AS review_observation,
  'a claimed control was tested against a negative sample' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the accountable owner confirmed the operating boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the current control objective", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for routine monitoring", "the source chronology was complete", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
