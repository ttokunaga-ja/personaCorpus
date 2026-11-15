-- Review period: 2026 Q3. This export risk snapshot control facts sql record was assessed using control boundary and operating ownership; the reviewer documented an ownership check.
-- Offline, static assurance review record.
SELECT
  'p03-full-008883' AS artifact_id,
  'p03-src-008883' AS source_record,
  'p03-secondary-04' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'How should a conflicting source be handled before the disposition is recorded?' AS evidence_question,
  'response coordinator retained timeline evidence for this review.' AS source_facts,
  'A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.' AS counterexample,
  'handover checklist' AS evidence_instrument,
  'the approved scope was not expanded beyond the available evidence' AS review_observation,
  'a scope boundary was compared with the evidence collection boundary' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the evidence custodian confirmed the record boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained with the decision record", "the exception path was not invoked", "the reviewer assessed a handover impact", "the decision was suitable for routine monitoring", "the source chronology contained a reconciled late entry", "the control claim was independently corroborated", "the review boundary excluded unavailable material", "the next assurance trigger is the scheduled review", "the disposition was communicated to the independent reviewer", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
