"""Synthetic ephemeral desktop working copy: p01-src-005628."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-7628"
    service: str = "svc-edge-api"
    latency_ms: int = 185
    state: str = "draft"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-005628", "working": is_within_alpha_reference(WorkingSample())})
