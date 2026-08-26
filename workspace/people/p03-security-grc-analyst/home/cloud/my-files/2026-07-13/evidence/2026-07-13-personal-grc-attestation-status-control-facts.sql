-- Cobalt Harbor Systems | Aegis Assurance | synthetic defensive record
-- Working file: 2026-07-13-personal-grc-attestation-status-control-facts-sql-s003048.sql | Topic: attestation status | Focus: action-owner acknowledgement
-- Purpose: organize a follow-up reference | Conclusion: For 2026-07-13, this working file organize a follow-up reference focused on action-owner acknowledgement; it supports a Medium residual-risk conclusion without asserting live-system access.
-- Ledger date anchors: 2026-07-13
-- Positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform | team.grc = Security GRC owns the AC-27 evidence register, risk treatment tracking, and audit-request coordination | date.2026-07-13 = Primary review date; bounded evidence intake opens 08:30 UTC and AC-27 review closes 16:45 UTC
-- Spine: org.cobalt-harbor; team.grc; rel.risk-031.treatment; date.2026-07-13; term.no-kio-claim
CREATE TABLE ac27_foundation_p03_full_003048 (
  artifact_id TEXT PRIMARY KEY, review_period TEXT NOT NULL, control_id TEXT NOT NULL,
  case_id TEXT NOT NULL, evidence_set INTEGER NOT NULL, timely_attestations INTEGER NOT NULL,
  review_population INTEGER NOT NULL, residual_risk TEXT NOT NULL, action_id TEXT NOT NULL
);
INSERT INTO ac27_foundation_p03_full_003048 VALUES ('p03-full-003048', '2026-07-13', 'AC-27', 'CASE-260713-184', 184, 47, 48, 'Medium', 'ACT-260713-011');
