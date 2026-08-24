"""Review evidence helper for CR-A7-006373."""

REVIEW_ID = 'CR-A7-006373'
REVIEW_ELAPSED_MINUTES = 400

def build_evidence() -> dict[str, object]:
    """Return synthetic review facts with no network operation."""
    return {"review_id": REVIEW_ID, "service": 'svc-event-bus', "focus": 'decision audit retention', "disposition": 'commented', "checks": (36, 34), "review_elapsed_minutes": REVIEW_ELAPSED_MINUTES}

if __name__ == "__main__":
    print(build_evidence())
