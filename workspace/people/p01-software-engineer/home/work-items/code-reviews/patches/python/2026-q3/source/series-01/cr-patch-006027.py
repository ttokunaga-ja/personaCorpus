"""Review evidence helper for CR-B4-006027."""

REVIEW_ID = 'CR-B4-006027'
REVIEW_ELAPSED_MINUTES = 400

def build_evidence() -> dict[str, object]:
    """Return synthetic review facts with no network operation."""
    return {"review_id": REVIEW_ID, "service": 'svc-decision-store', "focus": 'contract version visibility', "disposition": 'approved with note', "checks": (38, 36), "review_elapsed_minutes": REVIEW_ELAPSED_MINUTES}

if __name__ == "__main__":
    print(build_evidence())
