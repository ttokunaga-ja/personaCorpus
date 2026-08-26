-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using closure evidence and accountable disposition; the reviewer documented an ownership check.
-- Offline, static assurance review record.
SELECT
  'p03-full-005983' AS artifact_id,
  'p03-src-005983' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'How should a conflicting source be handled before the disposition is recorded?' AS evidence_question,
  'named control owner retained approval boundary for this review.' AS source_facts,
  'A repeated control statement does not establish operation when no counterexample was considered.' AS counterexample,
  'recertification record' AS evidence_instrument,
  'the local conclusion did not depend on a portal or live endpoint' AS review_observation,
  'a retained file was assessed for evidence freshness' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the current control objective", "the evidence was retained in the review register", "the exception path was considered and recorded", "the reviewer found no handover impact", "the decision was suitable for targeted follow-up", "the source chronology contained a reconciled late entry", "the control claim was corroborated by a compensating observation", "the review boundary excluded unavailable material", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
