"""Reference Standard P01-SRC-002138; Aster Loop synthetic engineering provenance."""
REFERENCE = {'id': 'REF-26Q3-002138', 'source_id': 'p01-src-002138', 'subject': 'Product Beta R4', 'anchor': 'Beta R4 contract review', 'reviewed_at': '2026-07-20', 'status': 'reviewed'}
def is_reviewed_reference(record: dict) -> bool:
    return record.get('status') == 'reviewed' and record.get('source_id') == 'p01-src-002138'
