-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using risk treatment and residual uncertainty; the reviewer confirmed the review conclusion.
-- Offline, static assurance review record.
SELECT
  'p03-full-009583' AS artifact_id,
  'p03-src-009583' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What decision can be made from the retained source material without external access?' AS evidence_question,
  'risk owner retained decision record for this review.' AS source_facts,
  'A file name is not a substitute for the source facts that support the conclusion.' AS counterexample,
  'training completion record' AS evidence_instrument,
  'the owner acknowledged a limitation without weakening traceability' AS review_observation,
  'a review result was compared with a counterexample' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the prior review conclusion", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer found no handover impact", "the decision was suitable for targeted follow-up", "the source chronology was complete", "the control claim was corroborated by a compensating observation", "the review boundary excluded unavailable material", "the next assurance trigger is the scheduled review", "the disposition was communicated to the independent reviewer", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
