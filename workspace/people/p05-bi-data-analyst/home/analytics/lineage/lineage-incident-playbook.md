# Lineage incident playbook

## Triage

1. Confirm the Synthetic OneDrive intake timestamp and connector diagnostic packet.
2. Compare the Warehouse S2 staging row count with the prior checkpoint.
3. Run the MTR-09 contract validator; isolate duplicate keys or negative JPY values.
4. Hold dashboard certification when freshness exceeds 180 minutes.

## Resolution record

Document incident owner, affected checkpoint, root transformation, remediation, and backfill confirmation. Never place production endpoints, credentials, or customer data in this record; this persona corpus uses synthetic data only.
