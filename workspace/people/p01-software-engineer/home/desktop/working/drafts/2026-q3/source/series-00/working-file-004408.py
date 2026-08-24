"""Ephemeral desktop draft for p01-src-004408; synthetic only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingObservation:
    issue_id: str = "ALW-5408"
    service: str = "svc-observability"
    latency_ms: int = 169
    state: str = "draft"

def within_alpha_reference(observation: WorkingObservation) -> bool:
    return observation.latency_ms <= 200

if __name__ == "__main__":
    current = WorkingObservation()
    print({"source": "p01-src-004408", "within_slo": within_alpha_reference(current)})
