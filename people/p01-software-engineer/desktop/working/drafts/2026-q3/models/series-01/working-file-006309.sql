-- Desktop working copy: p01-src-006309; state=working; synthetic only.
CREATE TEMP TABLE working_review_6309 (
  issue_id TEXT NOT NULL,
  service_name TEXT NOT NULL,
  observed_latency_ms INTEGER NOT NULL CHECK (observed_latency_ms > 0),
  review_date TEXT NOT NULL,
  accepted BOOLEAN NOT NULL DEFAULT FALSE
);
INSERT INTO working_review_6309 (issue_id, service_name, observed_latency_ms, review_date, accepted)
VALUES ('ALW-7309', 'svc-event-bus', 183, '2026-07-19', FALSE);
SELECT issue_id, service_name, observed_latency_ms
FROM working_review_6309
WHERE accepted = FALSE;
