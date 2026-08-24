-- Desktop working copy: p01-src-001482; state=review-pending; synthetic only.
CREATE TEMP TABLE working_review_1482 (
  issue_id TEXT NOT NULL,
  service_name TEXT NOT NULL,
  observed_latency_ms INTEGER NOT NULL CHECK (observed_latency_ms > 0),
  review_date TEXT NOT NULL,
  accepted BOOLEAN NOT NULL DEFAULT FALSE
);
INSERT INTO working_review_1482 (issue_id, service_name, observed_latency_ms, review_date, accepted)
VALUES ('ALW-2482', 'svc-edge-api', 166, '2026-08-25', FALSE);
SELECT issue_id, service_name, observed_latency_ms
FROM working_review_1482
WHERE accepted = FALSE;
