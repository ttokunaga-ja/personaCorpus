-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using risk treatment and residual uncertainty; the reviewer recorded a bounded exception.
-- Offline, static assurance review record.
SELECT
  'p03-full-007383' AS artifact_id,
  'p03-src-007383' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'How does the review distinguish complete evidence from an unsupported assertion?' AS evidence_question,
  'risk owner retained decision record for this review.' AS source_facts,
  'A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer.' AS counterexample,
  'retention schedule extract' AS evidence_instrument,
  'the review showed how contradictory evidence was resolved' AS review_observation,
  'a handoff record was assessed for continuity of evidence custody' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained in the review register", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for routine monitoring", "the source chronology was complete", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Reject unsupported claims and retain a concise explanation of the missing fact for the next review cycle.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
