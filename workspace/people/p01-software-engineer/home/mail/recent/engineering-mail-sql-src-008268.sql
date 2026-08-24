-- Synthetic mail-support query for MSG-2026Q3-008268; no customer or production data.
WITH recent_mail AS (
  SELECT 'MSG-2026Q3-008268' AS message_id, 'Release Coordinator' AS sender_role,
         'Observability Lead' AS recipient_role, 'Beta R4 schema freeze note' AS subject,
         'BETA-R4' AS project, 225 AS beta_design_p95_ms
)
SELECT message_id, sender_role, recipient_role, subject, project, beta_design_p95_ms
FROM recent_mail;
