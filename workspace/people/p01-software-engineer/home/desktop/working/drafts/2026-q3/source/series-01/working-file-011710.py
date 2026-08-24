"""Synthetic ephemeral desktop working copy: p01-src-011710."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-13710"
    service: str = "svc-observability"
    latency_ms: int = 191
    state: str = "review-pending"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-011710", "working": is_within_alpha_reference(WorkingSample())})
