"""Synthetic ephemeral desktop working copy: p01-src-007109."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-9109"
    service: str = "svc-contract-registry"
    latency_ms: int = 178
    state: str = "working"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-007109", "working": is_within_alpha_reference(WorkingSample())})
