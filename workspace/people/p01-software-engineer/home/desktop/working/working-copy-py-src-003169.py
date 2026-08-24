"""Ephemeral desktop draft for p01-src-003169; synthetic only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingObservation:
    issue_id: str = "ALW-4169"
    service: str = "svc-release-coordinator"
    latency_ms: int = 188
    state: str = "working"

def within_alpha_reference(observation: WorkingObservation) -> bool:
    return observation.latency_ms <= 200

if __name__ == "__main__":
    current = WorkingObservation()
    print({"source": "p01-src-003169", "within_slo": within_alpha_reference(current)})
