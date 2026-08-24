"""Synthetic ephemeral desktop working copy: p01-src-008331."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-10331"
    service: str = "svc-event-bus"
    latency_ms: int = 191
    state: str = "revised"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-008331", "working": is_within_alpha_reference(WorkingSample())})
