"""Product Beta R4 API contract API-B4-3852."""
from dataclasses import dataclass

@dataclass(frozen=True)
class Contract3852:
    contract_id: str = "API-B4-3852"
    schema_freeze: str = "2026-09-04"
    service: str = "svc-edge-api"
    tests_total: int = 1240
    tests_passed: int = 1231

    @property
    def tests_failed(self) -> int:
        return self.tests_total - self.tests_passed

if __name__ == "__main__":
    contract = Contract3852()
    assert contract.tests_failed == 9
    print(contract.contract_id)
