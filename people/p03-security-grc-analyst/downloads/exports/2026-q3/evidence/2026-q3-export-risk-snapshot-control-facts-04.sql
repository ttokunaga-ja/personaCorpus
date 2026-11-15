-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using source chronology and decision traceability; the reviewer confirmed the review conclusion.
-- Offline, static assurance review record.
SELECT
  'p03-full-006083' AS artifact_id,
  'p03-src-006083' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What decision can be made from the retained source material without external access?' AS evidence_question,
  'system steward retained review register for this review.' AS source_facts,
  'A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.' AS counterexample,
  'evidence freshness check' AS evidence_instrument,
  'the reviewer could locate the controlling decision without inference' AS review_observation,
  'a stale approval was compared with the current ownership assignment' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the prior review conclusion", "the evidence was retained in the review register", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for targeted follow-up", "the source chronology contained a reconciled late entry", "the control claim was corroborated by a compensating observation", "the review boundary excluded unavailable material", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
