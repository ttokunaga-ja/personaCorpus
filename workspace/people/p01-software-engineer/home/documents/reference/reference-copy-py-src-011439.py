"""Reference Standard P01-SRC-011439; Aster Loop synthetic engineering provenance."""
REFERENCE = {'id': 'REF-26Q3-011439', 'source_id': 'p01-src-011439', 'subject': 'Product Alpha R7', 'anchor': 'ADR-042 asynchronous release gate', 'reviewed_at': '2026-09-04', 'status': 'reviewed'}
def is_reviewed_reference(record: dict) -> bool:
    return record.get('status') == 'reviewed' and record.get('source_id') == 'p01-src-011439'
