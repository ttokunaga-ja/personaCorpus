"""Synthetic ephemeral desktop working copy: p01-src-005586."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-7586"
    service: str = "svc-edge-api"
    latency_ms: int = 174
    state: str = "review-pending"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-005586", "working": is_within_alpha_reference(WorkingSample())})
