-- Fully synthetic CRM export metadata for Northstar Cloud AE Rowan Mercer.
-- This document is read-only query text; it contains no credentials, connection
-- strings, DDL, DML, or production customer records.
-- Export as-of timestamp: 2026-08-24T09:00:00Z.

WITH export_manifest AS (
    SELECT '00-crm-opportunity-export.csv' AS file_name, 'csv' AS file_format, 8 AS data_row_count
    UNION ALL SELECT '01-account-contact-role-export.tsv', 'tsv', 10
    UNION ALL SELECT '02-forecast-category-export.csv', 'csv', 8
    UNION ALL SELECT '03-activity-history-export.tsv', 'tsv', 10
    UNION ALL SELECT '05-crm-report-export.eml', 'eml', 1
), export_context AS (
    SELECT
        'Northstar Cloud' AS organization_name,
        'Rowan Mercer' AS opportunity_owner,
        '2026-08-24T09:00:00Z' AS as_of_at,
        'All accounts, contacts, and domains are fictional .example.test data.' AS data_classification
)
SELECT
    manifest.file_name,
    manifest.file_format,
    manifest.data_row_count,
    context.organization_name,
    context.opportunity_owner,
    context.as_of_at,
    context.data_classification
FROM export_manifest AS manifest
CROSS JOIN export_context AS context
ORDER BY manifest.file_name;
