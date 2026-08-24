-- Desktop working copy: p01-src-002806; state=review-pending; synthetic only.
CREATE TEMP TABLE working_review_2806 (
  issue_id TEXT NOT NULL,
  service_name TEXT NOT NULL,
  observed_latency_ms INTEGER NOT NULL CHECK (observed_latency_ms > 0),
  review_date TEXT NOT NULL,
  accepted BOOLEAN NOT NULL DEFAULT FALSE
);
INSERT INTO working_review_2806 (issue_id, service_name, observed_latency_ms, review_date, accepted)
VALUES ('ALW-3806', 'svc-observability', 195, '2026-08-26', FALSE);
SELECT issue_id, service_name, observed_latency_ms
FROM working_review_2806
WHERE accepted = FALSE;
