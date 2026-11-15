"""Synthetic ephemeral desktop working copy: p01-src-009273."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-11273"
    service: str = "svc-event-bus"
    latency_ms: int = 172
    state: str = "working"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-009273", "working": is_within_alpha_reference(WorkingSample())})
