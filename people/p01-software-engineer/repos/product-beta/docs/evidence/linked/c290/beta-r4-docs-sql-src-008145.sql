-- BETA-DOC-008145: synthetic Beta R4 latency budget
CREATE TABLE IF NOT EXISTS beta_documentation_8145 (
  record_id TEXT PRIMARY KEY, service TEXT NOT NULL, status TEXT NOT NULL, latency_budget_ms INTEGER NOT NULL CHECK (latency_budget_ms = 225)
);
INSERT INTO beta_documentation_8145 (record_id, service, status, latency_budget_ms)
VALUES ('BETA-DOC-008145', 'svc-event-bus', 'draft', 225);
