"""Synthetic Product Beta R4 API contract API-B4-5879."""
from dataclasses import dataclass

@dataclass(frozen=True)
class Contract5879:
    identifier: str = "API-B4-5879"
    schema_freeze: str = "2026-09-04"
    domain: str = "compatibility-rule"
    total_tests: int = 1240
    passed_tests: int = 1231

    def failed_tests(self) -> int:
        return self.total_tests - self.passed_tests

if __name__ == "__main__":
    assert Contract5879().failed_tests() == 9
