-- Cobalt Harbor Systems | Aegis Assurance | synthetic defensive record
-- Working file: 2026-q3-personal-grc-review-calendar-control-facts-sql-s003218.sql | Topic: review calendar | Focus: approval separation
-- Purpose: compare a bounded control measure | Conclusion: For 2026-q3, this working file compare a bounded control measure focused on approval separation; it supports a Medium residual-risk conclusion without asserting live-system access.
-- Ledger date anchors: 
-- Positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform | action.act-260713-011 = Publish an AC-27 reviewer-completion register; completed 2026-07-15 | period.2026-q3 = Governance planning period from 2026-07-01 through 2026-09-30
-- Spine: org.cobalt-harbor; action.act-260713-011; rel.risk-031.treatment; period.2026-q3; term.no-kio-claim
CREATE TABLE ac27_foundation_p03_full_003218 (
  artifact_id TEXT PRIMARY KEY, review_period TEXT NOT NULL, control_id TEXT NOT NULL,
  case_id TEXT NOT NULL, evidence_set INTEGER NOT NULL, timely_attestations INTEGER NOT NULL,
  review_population INTEGER NOT NULL, residual_risk TEXT NOT NULL, action_id TEXT NOT NULL
);
INSERT INTO ac27_foundation_p03_full_003218 VALUES ('p03-full-003218', '2026-q3', 'AC-27', 'CASE-260713-184', 184, 47, 48, 'Medium', 'ACT-260713-011');
