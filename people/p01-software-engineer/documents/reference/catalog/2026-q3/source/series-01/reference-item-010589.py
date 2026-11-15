"""Reference Standard P01-SRC-010589; Aster Loop synthetic engineering provenance."""
REFERENCE = {'id': 'REF-26Q3-010589', 'source_id': 'p01-src-010589', 'subject': 'Product Alpha R7', 'anchor': 'ADR-042 asynchronous release gate', 'reviewed_at': '2026-07-20', 'status': 'reviewed'}
def is_reviewed_reference(record: dict) -> bool:
    return record.get('status') == 'reviewed' and record.get('source_id') == 'p01-src-010589'
