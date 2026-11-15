-- Review period: 2026 Q3. This vendor risk subprocessor map control facts sql record was assessed using exception ageing and escalation criteria; the reviewer documented an ownership check.
-- Offline, static assurance review record.
SELECT
  'p03-full-009449' AS artifact_id,
  'p03-src-009449' AS source_record,
  'p03-primary-08' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'How should a conflicting source be handled before the disposition is recorded?' AS evidence_question,
  'service owner retained contract evidence for this review.' AS source_facts,
  'A dependency reference is not meaningful unless the receiving artifact explains why it was consulted.' AS counterexample,
  'sampled access attestation' AS evidence_instrument,
  'the owner acknowledged a limitation without weakening traceability' AS review_observation,
  'a generic assurance phrase was rejected pending source facts' AS challenge_performed,
  '["the evidence set was collected during the follow-up review", "the accountable owner confirmed the operating boundary", "the reviewer sampled a routine operating condition", "the source fact supported acceptance with a documented caveat", "the comparison used the prior review conclusion", "the evidence was retained with the decision record", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for routine monitoring", "the source chronology was complete", "the control claim was corroborated by a compensating observation", "the review boundary excluded unavailable material", "the next assurance trigger is the scheduled review", "the disposition was communicated to the independent reviewer", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.' AS decision_rule,
  'escalated for corroboration with the decision basis retained' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
