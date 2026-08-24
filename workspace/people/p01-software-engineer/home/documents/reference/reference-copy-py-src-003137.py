"""Reference Standard P01-SRC-003137; Aster Loop synthetic engineering provenance."""
REFERENCE = {'id': 'REF-26Q3-003137', 'source_id': 'p01-src-003137', 'subject': 'Product Alpha R7', 'anchor': 'ADR-042 asynchronous release gate', 'reviewed_at': '2026-07-20', 'status': 'reviewed'}
def is_reviewed_reference(record: dict) -> bool:
    return record.get('status') == 'reviewed' and record.get('source_id') == 'p01-src-003137'
