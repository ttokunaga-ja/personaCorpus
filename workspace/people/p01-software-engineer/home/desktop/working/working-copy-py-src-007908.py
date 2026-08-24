"""Synthetic ephemeral desktop working copy: p01-src-007908."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-9908"
    service: str = "svc-edge-api"
    latency_ms: int = 171
    state: str = "draft"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-007908", "working": is_within_alpha_reference(WorkingSample())})
