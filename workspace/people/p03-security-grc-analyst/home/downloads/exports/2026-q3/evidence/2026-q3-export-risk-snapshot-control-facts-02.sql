-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using source chronology and decision traceability; the reviewer preserved an escalation decision.
-- Offline, static assurance review record.
SELECT
  'p03-full-005283' AS artifact_id,
  'p03-src-005283' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'Which responsibility boundary must be visible before the record is approved?' AS evidence_question,
  'system steward retained review register for this review.' AS source_facts,
  'A file name is not a substitute for the source facts that support the conclusion.' AS counterexample,
  'contract obligation extract' AS evidence_instrument,
  'the conclusion matched the scope of the stated control' AS review_observation,
  'an absent relationship was made explicit before acceptance' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the prior review conclusion", "the evidence was retained with the decision record", "the exception path was not invoked", "the reviewer assessed a handover impact", "the decision was suitable for routine monitoring", "the source chronology was complete", "the control claim was corroborated by a compensating observation", "the review boundary excluded unavailable material", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
