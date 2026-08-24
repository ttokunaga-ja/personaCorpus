"""Synthetic Product Alpha R7 architecture utility: p01-src-000155."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ArchitectureObservation:
    record_id: str
    service: str
    decision_id: str
    p95_ms: int
    def meets_slo(self) -> bool:
        return self.p95_ms <= 200

def build_observation() -> ArchitectureObservation:
    return ArchitectureObservation("ARCH-A7-000155", "svc-contract-registry", "ADR-156", 179)

if __name__ == "__main__":
    print(build_observation().meets_slo())
