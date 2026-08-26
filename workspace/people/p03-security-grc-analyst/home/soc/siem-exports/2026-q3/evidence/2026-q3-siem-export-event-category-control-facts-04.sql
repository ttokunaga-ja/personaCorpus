-- Review period: 2026 Q3. This siem export event category control facts sql record was assessed using evidence custody and reviewer independence; the reviewer identified a corroboration need.
-- Offline, static assurance review record.
SELECT
  'p03-full-008848' AS artifact_id,
  'p03-src-008848' AS source_record,
  'p03-primary-09' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'Which source fact would cause the reviewer to accept the control conclusion?' AS evidence_question,
  'records owner retained disposition record for this review.' AS source_facts,
  'A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.' AS counterexample,
  'versioned review register' AS evidence_instrument,
  'the retained fact predated the decision by one review cycle' AS review_observation,
  'an unassigned exception was tested against the escalation route' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the accountable owner confirmed the operating boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained in the review register", "the exception path was not invoked", "the reviewer assessed a handover impact", "the decision was suitable for routine monitoring", "the source chronology contained a reconciled late entry", "the control claim was independently corroborated", "the review boundary excluded unavailable material", "the next assurance trigger is the scheduled review", "the disposition was communicated to the independent reviewer", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
