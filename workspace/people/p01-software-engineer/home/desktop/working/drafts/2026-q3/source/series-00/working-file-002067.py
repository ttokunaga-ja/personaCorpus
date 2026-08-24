"""Ephemeral desktop draft for p01-src-002067; synthetic only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingObservation:
    issue_id: str = "ALW-3067"
    service: str = "svc-event-bus"
    latency_ms: int = 196
    state: str = "revised"

def within_alpha_reference(observation: WorkingObservation) -> bool:
    return observation.latency_ms <= 200

if __name__ == "__main__":
    current = WorkingObservation()
    print({"source": "p01-src-002067", "within_slo": within_alpha_reference(current)})
