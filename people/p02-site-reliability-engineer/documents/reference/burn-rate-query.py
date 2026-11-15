#!/usr/bin/env python3
"""Calculate an SLO burn rate from a completed observation window.

This utility is intentionally read-only: it accepts counts and emits JSON to
standard output, making it suitable for a local dashboard export or an alert
evaluation preview.
"""

from __future__ import annotations

import argparse
import json


def calculate_burn_rate(total_events: int, failed_events: int, slo_target: float) -> dict[str, float | int]:
    """Return observed error rate and burn rate for one measurement window."""
    if total_events <= 0:
        raise ValueError("total_events must be greater than zero")
    if not 0 < slo_target < 1:
        raise ValueError("slo_target must be between zero and one")
    if not 0 <= failed_events <= total_events:
        raise ValueError("failed_events must be between zero and total_events")

    observed_error_rate = failed_events / total_events
    allowed_error_rate = 1 - slo_target
    return {
        "total_events": total_events,
        "failed_events": failed_events,
        "observed_error_rate": round(observed_error_rate, 8),
        "allowed_error_rate": round(allowed_error_rate, 8),
        "burn_rate": round(observed_error_rate / allowed_error_rate, 4),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Calculate an SLO burn rate.")
    parser.add_argument("--total-events", type=int, required=True)
    parser.add_argument("--failed-events", type=int, required=True)
    parser.add_argument("--slo-target", type=float, default=0.99)
    args = parser.parse_args()
    print(json.dumps(calculate_burn_rate(args.total_events, args.failed_events, args.slo_target), indent=2))


if __name__ == "__main__":
    main()
