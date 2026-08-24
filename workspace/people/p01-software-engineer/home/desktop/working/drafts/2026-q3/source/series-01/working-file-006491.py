"""Synthetic ephemeral desktop working copy: p01-src-006491."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingSample:
    issue: str = "ALW-8491"
    service: str = "svc-contract-registry"
    latency_ms: int = 180
    state: str = "revised"

def is_within_alpha_reference(sample: WorkingSample) -> bool:
    return sample.latency_ms <= 200

if __name__ == "__main__":
    print({"source": "p01-src-006491", "working": is_within_alpha_reference(WorkingSample())})
