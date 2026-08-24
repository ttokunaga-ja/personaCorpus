-- Synthetic mail-support query for MSG-2026Q3-009068; no customer or production data.
WITH recent_mail AS (
  SELECT 'MSG-2026Q3-009068' AS message_id, 'Observability Lead' AS sender_role,
         'Migration Lead' AS recipient_role, 'latency budget reconciliation' AS subject,
         'BETA-R4' AS project, 225 AS beta_design_p95_ms
)
SELECT message_id, sender_role, recipient_role, subject, project, beta_design_p95_ms
FROM recent_mail;
