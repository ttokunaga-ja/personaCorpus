-- Review period: 2026 Q3. This audit request reviewer note control facts sql record was assessed using retention basis and follow-up condition; the reviewer confirmed the review conclusion.
-- Offline, static assurance review record.
SELECT
  'p03-full-002514' AS artifact_id,
  'p03-src-002514' AS source_record,
  'p03-primary-07' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'What decision can be made from the retained source material without external access?' AS evidence_question,
  'change sponsor retained implementation record for this review.' AS source_facts,
  'A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.' AS counterexample,
  'meeting decision minute' AS evidence_instrument,
  'the owner and custodian were named consistently' AS review_observation,
  'an unassigned exception was tested against the escalation route' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the evidence custodian confirmed the record boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained in the review register", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for targeted follow-up", "the source chronology was complete", "the control claim was independently corroborated", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is the scheduled review", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
