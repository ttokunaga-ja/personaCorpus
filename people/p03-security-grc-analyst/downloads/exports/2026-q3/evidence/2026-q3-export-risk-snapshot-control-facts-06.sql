-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using retention basis and follow-up condition; the reviewer retained a challenge response.
-- Offline, static assurance review record.
SELECT
  'p03-full-008183' AS artifact_id,
  'p03-src-008183' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What minimal trace proves that the control remains attributable to its owner?' AS evidence_question,
  'change sponsor retained implementation record for this review.' AS source_facts,
  'A dependency reference is not meaningful unless the receiving artifact explains why it was consulted.' AS counterexample,
  'exception ageing report' AS evidence_instrument,
  'the record explained the relationship between policy and practice' AS review_observation,
  'a change record was tested for reviewer independence' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for targeted follow-up", "the source chronology contained a reconciled late entry", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
