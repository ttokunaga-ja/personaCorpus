"""Synthetic ephemeral desktop working copy: p01-src-009209."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-11209"
    service: str = "svc-contract-registry"
    latency_ms: int = 170
    state: str = "working"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-009209", "working": is_within_alpha_reference(WorkingSample())})
