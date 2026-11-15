-- Desktop working copy: p01-src-002984; state=draft; synthetic only.
CREATE TEMP TABLE working_review_2984 (
  issue_id TEXT NOT NULL,
  service_name TEXT NOT NULL,
  observed_latency_ms INTEGER NOT NULL CHECK (observed_latency_ms > 0),
  review_date TEXT NOT NULL,
  accepted BOOLEAN NOT NULL DEFAULT FALSE
);
INSERT INTO working_review_2984 (issue_id, service_name, observed_latency_ms, review_date, accepted)
VALUES ('ALW-3984', 'svc-decision-store', 188, '2026-06-15', FALSE);
SELECT issue_id, service_name, observed_latency_ms
FROM working_review_2984
WHERE accepted = FALSE;
