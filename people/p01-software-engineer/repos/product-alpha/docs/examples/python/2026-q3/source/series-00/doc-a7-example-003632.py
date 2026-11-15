"""Synthetic Product Alpha R7 documentation helper: runbook navigation."""
ARTIFACT_ID = 'p01-full-003632'
SERVICE = 'svc-decision-store'
def release_summary() -> dict[str, int | str]:
    return {"artifact_id": ARTIFACT_ID, "service": SERVICE, "release": "ALPHA-R7", "p95_ms": 184, "slo_ceiling_ms": 200, "approved": 8883, "non_approved": 117}
if __name__ == "__main__":
    print(release_summary())
