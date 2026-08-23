from collections import Counter

def backport_report(fixtures):
    """Summarize synthetic PR fixtures without contacting a source-control service."""
    states = Counter(item["state"] for item in fixtures)
    eligible = [item["id"] for item in fixtures if item["state"] == "merged" and item["target"] == "r6"]
    return {
        "fixture_count": len(fixtures),
        "states": dict(sorted(states.items())),
        "eligible_backports": sorted(eligible),
        "decision_ref": "ADR-042",
    }
