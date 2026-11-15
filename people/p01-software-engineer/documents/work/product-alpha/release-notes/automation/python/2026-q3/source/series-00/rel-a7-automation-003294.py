"""Synthetic Product Alpha R7 release-note helper: p01-full-003294."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ReleaseNote:
    record_id: str
    observed_p95_ms: int = 184
    slo_ceiling_ms: int = 200

def release_note() -> ReleaseNote:
    note = ReleaseNote('REL-A7-20260719-94')
    if note.observed_p95_ms > note.slo_ceiling_ms:
        raise ValueError('synthetic latency ceiling exceeded')
    return note

if __name__ == '__main__':
    print(release_note())
