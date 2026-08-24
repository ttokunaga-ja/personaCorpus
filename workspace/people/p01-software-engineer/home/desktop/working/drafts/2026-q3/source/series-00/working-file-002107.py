"""Ephemeral desktop draft for p01-src-002107; synthetic only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingObservation:
    issue_id: str = "ALW-3107"
    service: str = "svc-release-coordinator"
    latency_ms: int = 199
    state: str = "revised"

def within_alpha_reference(observation: WorkingObservation) -> bool:
    return observation.latency_ms <= 200

if __name__ == "__main__":
    current = WorkingObservation()
    print({"source": "p01-src-002107", "within_slo": within_alpha_reference(current)})
