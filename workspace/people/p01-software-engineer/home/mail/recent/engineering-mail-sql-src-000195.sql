-- Synthetic mail-support query for MSG-2026Q3-000195; no customer or production data.
WITH recent_mail AS (
  SELECT 'MSG-2026Q3-000195' AS message_id, 'API Steward' AS sender_role,
         'Decision Owner' AS recipient_role, 'staging rollout observation' AS subject,
         'BETA-R4' AS project, 225 AS beta_design_p95_ms
)
SELECT message_id, sender_role, recipient_role, subject, project, beta_design_p95_ms
FROM recent_mail;
