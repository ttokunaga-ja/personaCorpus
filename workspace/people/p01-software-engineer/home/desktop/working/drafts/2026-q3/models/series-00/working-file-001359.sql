-- Desktop working copy: p01-src-001359; state=revised; synthetic only.
CREATE TEMP TABLE working_review_1359 (
  issue_id TEXT NOT NULL,
  service_name TEXT NOT NULL,
  observed_latency_ms INTEGER NOT NULL CHECK (observed_latency_ms > 0),
  review_date TEXT NOT NULL,
  accepted BOOLEAN NOT NULL DEFAULT FALSE
);
INSERT INTO working_review_1359 (issue_id, service_name, observed_latency_ms, review_date, accepted)
VALUES ('ALW-2359', 'svc-event-bus', 191, '2026-09-10', FALSE);
SELECT issue_id, service_name, observed_latency_ms
FROM working_review_1359
WHERE accepted = FALSE;
