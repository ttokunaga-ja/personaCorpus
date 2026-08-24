"""Review evidence helper for CR-A7-006227."""

REVIEW_ID = 'CR-A7-006227'
REVIEW_ELAPSED_MINUTES = 400

def build_evidence() -> dict[str, object]:
    """Return synthetic review facts with no network operation."""
    return {"review_id": REVIEW_ID, "service": 'svc-decision-store', "focus": 'contract version visibility', "disposition": 'approved with note', "checks": (35, 33), "review_elapsed_minutes": REVIEW_ELAPSED_MINUTES}

if __name__ == "__main__":
    print(build_evidence())
