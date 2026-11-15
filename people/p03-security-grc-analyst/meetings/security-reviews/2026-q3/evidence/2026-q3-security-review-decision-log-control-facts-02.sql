-- Review period: 2026 Q3. This review decision control facts sql record was assessed using retention basis and follow-up condition; the reviewer accepted the retained basis.
-- Offline, static assurance review record.
SELECT
  'p03-full-008082' AS artifact_id,
  'p03-src-008082' AS source_record,
  'p03-primary-12' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What observable evidence demonstrates that the stated control objective is met?' AS evidence_question,
  'change sponsor retained implementation record for this review.' AS source_facts,
  'A complete-looking register is not reliable if the decision rule and disposition are absent.' AS counterexample,
  'control-to-evidence map' AS evidence_instrument,
  'the reviewer could locate the controlling decision without inference' AS review_observation,
  'a broad claim was narrowed to the observable control action' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the evidence custodian confirmed the record boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained in the review register", "the exception path was not invoked", "the reviewer assessed a handover impact", "the decision was suitable for targeted follow-up", "the source chronology contained a reconciled late entry", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
