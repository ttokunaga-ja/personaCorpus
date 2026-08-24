"""Synthetic Product Alpha R7 release helper (p01-full-006594)."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ReleaseEvidence:
    record_id: str
    observed_p95_ms: int = 184
    slo_ceiling_ms: int = 200

def evidence() -> ReleaseEvidence:
    item = ReleaseEvidence('REL-A7-20260715-96')
    if item.observed_p95_ms > item.slo_ceiling_ms:
        raise RuntimeError('synthetic release SLO exceeded')
    return item

if __name__ == '__main__':
    print(evidence())
