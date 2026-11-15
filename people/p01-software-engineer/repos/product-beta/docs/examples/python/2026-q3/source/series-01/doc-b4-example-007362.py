"""BETA-REPO-007362: Beta R4 operator guide checker."""
from dataclasses import dataclass

@dataclass(frozen=True)
class Evidence:
    total: int = 1240
    passed: int = 1231
    failed: int = 9

def reconcile() -> dict[str, object]:
    evidence = Evidence()
    if evidence.total != evidence.passed + evidence.failed:
        raise ValueError('contract counts must reconcile')
    return {'record_id': 'BETA-REPO-007362', 'service': 'svc-edge-api', 'pass_rate_percent': round(evidence.passed / evidence.total * 100, 2)}
