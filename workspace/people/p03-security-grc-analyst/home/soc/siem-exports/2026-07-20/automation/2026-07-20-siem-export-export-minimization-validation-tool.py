"""Cobalt Harbor Systems Sentinel Export; evidence set 184; EV-184-07; export minimization; 2026-07-20; synthetic minimized defensive SIEM context. Spine anchors: org.cobalt-harbor, term.minimized-export, rel.siem.minimization, date.2026-07-20, term.no-kio-claim."""
ARTIFACT_ID = "p03-full-002405"
SOURCE_ID = "p03-src-002405"
RECORD_SEQUENCE = 2405

def validate_minimized_count(category: str, count: int) -> bool:
    return category in {"access-review", "approval-record", "checksum-reconciliation"} and isinstance(count, int) and count >= 0
