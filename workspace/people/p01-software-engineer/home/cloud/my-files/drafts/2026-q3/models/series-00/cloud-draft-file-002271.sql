-- Draft working-copy query for BETA-R4-CLOUD-002271
-- State: draft; synthetic Product Beta R4 evidence
SELECT 'p01-src-002271' AS source_id, 'BETA-R4-CLOUD-002271' AS cloud_record_id, 'draft' AS lifecycle_state,
       1240 AS total_checks, 1231 AS passed_checks, 9 AS failed_checks, 225 AS design_target_p95_ms
WHERE 1240 = 1231 + 9;
