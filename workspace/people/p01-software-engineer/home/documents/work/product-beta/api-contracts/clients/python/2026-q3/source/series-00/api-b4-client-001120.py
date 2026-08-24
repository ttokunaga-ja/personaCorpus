"""Product Beta R4 API contract API-B4-1120."""
from dataclasses import dataclass

@dataclass(frozen=True)
class Contract1120:
    contract_id: str = "API-B4-1120"
    schema_freeze: str = "2026-09-04"
    service: str = "svc-event-bus"
    tests_total: int = 1240
    tests_passed: int = 1231

    @property
    def tests_failed(self) -> int:
        return self.tests_total - self.tests_passed

if __name__ == "__main__":
    contract = Contract1120()
    assert contract.tests_failed == 9
    print(contract.contract_id)
