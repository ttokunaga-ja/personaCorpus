"""BETA-REPO-011407: Beta R4 compatibility note checker."""
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
    return {'record_id': 'BETA-REPO-011407', 'service': 'svc-release-coordinator', 'pass_rate_percent': round(evidence.passed / evidence.total * 100, 2)}
