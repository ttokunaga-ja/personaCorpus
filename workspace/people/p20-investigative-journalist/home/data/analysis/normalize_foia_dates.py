"""Normalize synthetic FOIA dates without network or credential access."""
from datetime import date


def response_days(received: str, verified: str) -> int:
    """Return calendar days between ISO-8601 synthetic dates."""
    return (date.fromisoformat(verified) - date.fromisoformat(received)).days


EXAMPLE_REQUESTS = (
    {"request_id": "CL-SA26-FOIA-17", "received": "2026-08-14", "verified": "2026-08-19"},
    {"request_id": "CL-SB26-FOIA-17", "received": "2026-08-15", "verified": "2026-08-19"},
)


if __name__ == "__main__":
    for request in EXAMPLE_REQUESTS:
        print(request["request_id"], response_days(request["received"], request["verified"]))
