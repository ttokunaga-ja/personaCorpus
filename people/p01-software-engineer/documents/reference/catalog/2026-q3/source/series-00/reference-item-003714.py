"""Reference Standard P01-SRC-003714; Aster Loop synthetic engineering provenance."""
REFERENCE = {'id': 'REF-26Q3-003714', 'source_id': 'p01-src-003714', 'subject': 'Product Beta R4', 'anchor': 'Beta R4 contract review', 'reviewed_at': '2026-09-04', 'status': 'reviewed'}
def is_reviewed_reference(record: dict) -> bool:
    return record.get('status') == 'reviewed' and record.get('source_id') == 'p01-src-003714'
