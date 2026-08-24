"""Review evidence helper for CR-A7-011071."""

REVIEW_ID = 'CR-A7-011071'
REVIEW_ELAPSED_MINUTES = 400

def build_evidence() -> dict[str, object]:
    """Return synthetic review facts with no network operation."""
    return {"review_id": REVIEW_ID, "service": 'svc-release-coordinator', "focus": 'bounded retry semantics', "disposition": 'changes requested', "checks": (36, 34), "review_elapsed_minutes": REVIEW_ELAPSED_MINUTES}

if __name__ == "__main__":
    print(build_evidence())
