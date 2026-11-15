"""Synthetic ephemeral desktop working copy: p01-src-009673."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-11673"
    service: str = "svc-release-coordinator"
    latency_ms: int = 169
    state: str = "working"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-009673", "working": is_within_alpha_reference(WorkingSample())})
