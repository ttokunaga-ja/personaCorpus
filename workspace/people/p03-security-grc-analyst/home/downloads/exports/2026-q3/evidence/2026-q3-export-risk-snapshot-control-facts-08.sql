-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using control boundary and operating ownership; the reviewer retained a challenge response.
-- Offline, static assurance review record.
SELECT
  'p03-full-008819' AS artifact_id,
  'p03-src-008819' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What minimal trace proves that the control remains attributable to its owner?' AS evidence_question,
  'response coordinator retained timeline evidence for this review.' AS source_facts,
  'A file name is not a substitute for the source facts that support the conclusion.' AS counterexample,
  'quality review annotation' AS evidence_instrument,
  'the record explained the relationship between policy and practice' AS review_observation,
  'an unsupported completion status was tested for underlying proof' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer found no handover impact", "the decision was suitable for routine monitoring", "the source chronology contained a reconciled late entry", "the control claim was independently corroborated", "the review boundary excluded unavailable material", "the next assurance trigger is the scheduled review", "the disposition was communicated to the independent reviewer", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Treat the control as effective only when the counterexample has been considered and the exception path is explicit.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
