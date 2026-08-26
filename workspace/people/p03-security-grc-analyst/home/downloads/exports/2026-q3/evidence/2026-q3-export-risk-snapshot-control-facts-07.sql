-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using evidence custody and reviewer independence; the reviewer preserved an escalation decision.
-- Offline, static assurance review record.
SELECT
  'p03-full-008783' AS artifact_id,
  'p03-src-008783' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'Which responsibility boundary must be visible before the record is approved?' AS evidence_question,
  'records owner retained disposition record for this review.' AS source_facts,
  'A repeated control statement does not establish operation when no counterexample was considered.' AS counterexample,
  'evidence freshness check' AS evidence_instrument,
  'the reviewer could locate the controlling decision without inference' AS review_observation,
  'a late evidence submission was separated from the original decision' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the prior review conclusion", "the evidence was retained in the review register", "the exception path was considered and recorded", "the reviewer found no handover impact", "the decision was suitable for routine monitoring", "the source chronology contained a reconciled late entry", "the control claim was independently corroborated", "the review boundary excluded unavailable material", "the next assurance trigger is the scheduled review", "the disposition was communicated to the independent reviewer", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
