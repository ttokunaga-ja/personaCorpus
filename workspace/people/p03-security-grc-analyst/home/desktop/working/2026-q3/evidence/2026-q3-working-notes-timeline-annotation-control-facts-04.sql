-- Review period: 2026 Q3. This working notes timeline annotation control facts sql record was assessed using risk treatment and residual uncertainty; the reviewer set a follow-up assurance trigger.
-- Offline, static assurance review record.
SELECT
  'p03-full-008049' AS artifact_id,
  'p03-src-008049' AS source_record,
  'p03-secondary-01' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'Which review question exposes a gap between policy language and operating evidence?' AS evidence_question,
  'risk owner retained decision record for this review.' AS source_facts,
  'A file name is not a substitute for the source facts that support the conclusion.' AS counterexample,
  'evidence freshness check' AS evidence_instrument,
  'the owner acknowledged a limitation without weakening traceability' AS review_observation,
  'a corrective action was tested for closure evidence' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the accountable owner confirmed the operating boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer found no handover impact", "the decision was suitable for targeted follow-up", "the source chronology contained a reconciled late entry", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
