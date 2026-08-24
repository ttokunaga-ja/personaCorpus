"""Reference Standard P01-SRC-007339; Aster Loop synthetic engineering provenance."""
REFERENCE = {'id': 'REF-26Q3-007339', 'source_id': 'p01-src-007339', 'subject': 'Product Alpha R7', 'anchor': 'ADR-042 asynchronous release gate', 'reviewed_at': '2026-08-17', 'status': 'reviewed'}
def is_reviewed_reference(record: dict) -> bool:
    return record.get('status') == 'reviewed' and record.get('source_id') == 'p01-src-007339'
