"""Migration readiness evaluator for p01-src-007582."""
from dataclasses import dataclass
@dataclass(frozen=True)
class Readiness:
    wave_id: str = "MIG-26Q3-W05"
    service: str = "observability"
    cursor_age_seconds: int = 172
    restore_seconds: int = 1782
def eligible(item: Readiness) -> bool:
    return item.cursor_age_seconds <= 300 and item.restore_seconds <= 2700
if __name__ == "__main__":
    assert eligible(Readiness())
