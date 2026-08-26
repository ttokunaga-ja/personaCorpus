-- Cobalt Harbor Systems | Aegis Assurance | synthetic defensive record
-- Working file: 2026-07-13-personal-grc-action-tracker-control-facts-sql-s001681.sql | Topic: action tracker | Focus: action-owner acknowledgement
-- Purpose: document a review handoff | Conclusion: For 2026-07-13, this working file document a review handoff focused on action-owner acknowledgement; it supports a Medium residual-risk conclusion without asserting live-system access.
-- Ledger date anchors: 2026-07-13
-- Positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform | measure.timely-attestation = Timely attestation rate before compensating review: 97.9% (47 of 48) | date.2026-07-13 = Primary review date; bounded evidence intake opens 08:30 UTC and AC-27 review closes 16:45 UTC
-- Spine: org.cobalt-harbor; measure.timely-attestation; rel.risk-031.treatment; date.2026-07-13; term.no-kio-claim
CREATE TABLE ac27_foundation_p03_full_001681 (
  artifact_id TEXT PRIMARY KEY, review_period TEXT NOT NULL, control_id TEXT NOT NULL,
  case_id TEXT NOT NULL, evidence_set INTEGER NOT NULL, timely_attestations INTEGER NOT NULL,
  review_population INTEGER NOT NULL, residual_risk TEXT NOT NULL, action_id TEXT NOT NULL
);
INSERT INTO ac27_foundation_p03_full_001681 VALUES ('p03-full-001681', '2026-07-13', 'AC-27', 'CASE-260713-184', 184, 47, 48, 'Medium', 'ACT-260713-011');
