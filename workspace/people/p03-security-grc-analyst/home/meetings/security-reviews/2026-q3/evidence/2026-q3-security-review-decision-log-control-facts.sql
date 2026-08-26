-- Review period: 2026 Q3. This review decision control facts sql record was assessed using access attestation and compensating action; the reviewer confirmed the review conclusion.
-- Offline, static assurance review record.
SELECT
  'p03-full-006682' AS artifact_id,
  'p03-src-006682' AS source_record,
  'p03-primary-12' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What decision can be made from the retained source material without external access?' AS evidence_question,
  'detection owner retained coverage record for this review.' AS source_facts,
  'A repeated control statement does not establish operation when no counterexample was considered.' AS counterexample,
  'versioned review register' AS evidence_instrument,
  'the exception route was visible to an independent reviewer' AS review_observation,
  'an exception rationale was tested for a documented decision rule' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the evidence custodian confirmed the record boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the current control objective", "the evidence was retained in the review register", "the exception path was not invoked", "the reviewer found no handover impact", "the decision was suitable for routine monitoring", "the source chronology contained a reconciled late entry", "the control claim was independently corroborated", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
