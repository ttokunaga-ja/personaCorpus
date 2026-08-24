"""Migration readiness evaluator for p01-src-003054."""
from dataclasses import dataclass
@dataclass(frozen=True)
class Readiness:
    wave_id: str = "MIG-26Q3-W01"
    service: str = "media-worker"
    cursor_age_seconds: int = 204
    restore_seconds: int = 1454
def eligible(item: Readiness) -> bool:
    return item.cursor_age_seconds <= 300 and item.restore_seconds <= 2700
if __name__ == "__main__":
    assert eligible(Readiness())
