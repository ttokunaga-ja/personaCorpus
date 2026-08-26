-- Review period: 2026 Q3. This intake request attachment control facts sql record was assessed using retention basis and follow-up condition; the reviewer recorded a bounded exception.
-- Offline, static assurance review record.
SELECT
  'p03-full-006716' AS artifact_id,
  'p03-src-006716' AS source_record,
  'p03-secondary-03' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'How does the review distinguish complete evidence from an unsupported assertion?' AS evidence_question,
  'change sponsor retained implementation record for this review.' AS source_facts,
  'A complete-looking register is not reliable if the decision rule and disposition are absent.' AS counterexample,
  'approval trail summary' AS evidence_instrument,
  'the rationale accounted for an incomplete sample' AS review_observation,
  'a summary was checked against the underlying decision minute' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the accountable owner confirmed the operating boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the current control objective", "the evidence was retained with the decision record", "the exception path was not invoked", "the reviewer found no handover impact", "the decision was suitable for routine monitoring", "the source chronology contained a reconciled late entry", "the control claim was independently corroborated", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
