"""Product Beta R4 API contract API-B4-4525."""
from dataclasses import dataclass

@dataclass(frozen=True)
class Contract4525:
    contract_id: str = "API-B4-4525"
    schema_freeze: str = "2026-09-04"
    service: str = "svc-contract-registry"
    tests_total: int = 1240
    tests_passed: int = 1231

    @property
    def tests_failed(self) -> int:
        return self.tests_total - self.tests_passed

if __name__ == "__main__":
    contract = Contract4525()
    assert contract.tests_failed == 9
    print(contract.contract_id)
