-- Cobalt Harbor Systems | Aegis Assurance | synthetic defensive record
-- Working file: 2026-07-14-personal-grc-action-tracker-control-facts-sql-s009285.sql | Topic: action tracker | Focus: residual-risk statement
-- Purpose: capture a non-operational assurance observation | Conclusion: For 2026-07-14, this working file capture a non-operational assurance observation focused on residual-risk statement; it supports a Medium residual-risk conclusion without asserting live-system access.
-- Ledger date anchors: 2026-07-14
-- Positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform | team.grc = Security GRC owns the AC-27 evidence register, risk treatment tracking, and audit-request coordination | date.2026-07-14 = Evidence normalization, checksum confirmation, and action-owner acknowledgement
-- Spine: org.cobalt-harbor; team.grc; rel.risk-031.treatment; date.2026-07-14; term.no-kio-claim
CREATE TABLE ac27_foundation_p03_full_009285 (
  artifact_id TEXT PRIMARY KEY, review_period TEXT NOT NULL, control_id TEXT NOT NULL,
  case_id TEXT NOT NULL, evidence_set INTEGER NOT NULL, timely_attestations INTEGER NOT NULL,
  review_population INTEGER NOT NULL, residual_risk TEXT NOT NULL, action_id TEXT NOT NULL
);
INSERT INTO ac27_foundation_p03_full_009285 VALUES ('p03-full-009285', '2026-07-14', 'AC-27', 'CASE-260713-184', 184, 47, 48, 'Medium', 'ACT-260713-011');
