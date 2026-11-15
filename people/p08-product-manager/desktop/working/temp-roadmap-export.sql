-- Juniper Works / Product Alpha Q3
-- Synthetic roadmap export only. Read-only query; no credentials or production identifiers.
-- Schema notes:
--   synthetic_workspaces(workspace_id, cohort_name, activated_at, adoption_rate)
--   roadmap_initiatives(initiative_id, initiative_name, target_milestone, status, owner_role)
--   initiative_workspace_rollup(initiative_id, workspace_count, adoption_rate)

WITH cohort AS (
  SELECT workspace_id, adoption_rate
  FROM synthetic_workspaces
  WHERE cohort_name = 'Product Alpha Q3'
), roadmap AS (
  SELECT initiative_id, initiative_name, target_milestone, status, owner_role
  FROM roadmap_initiatives
  WHERE target_milestone = 'M-27'
)
SELECT
  roadmap.initiative_name,
  roadmap.target_milestone,
  roadmap.status,
  roadmap.owner_role,
  rollup.workspace_count,
  ROUND(rollup.adoption_rate * 100, 1) AS adoption_rate_pct,
  26.0 AS q3_target_pct
FROM roadmap
LEFT JOIN initiative_workspace_rollup AS rollup
  ON roadmap.initiative_id = rollup.initiative_id
ORDER BY adoption_rate_pct ASC, roadmap.initiative_name;
