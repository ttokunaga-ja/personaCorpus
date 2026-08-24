"""Migration readiness evaluator for p01-src-003391."""
from dataclasses import dataclass
@dataclass(frozen=True)
class Readiness:
    wave_id: str = "MIG-26Q3-W02"
    service: str = "catalog-index"
    cursor_age_seconds: int = 61
    restore_seconds: int = 1791
def eligible(item: Readiness) -> bool:
    return item.cursor_age_seconds <= 300 and item.restore_seconds <= 2700
if __name__ == "__main__":
    assert eligible(Readiness())
