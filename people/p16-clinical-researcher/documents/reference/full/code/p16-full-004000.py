"""Synthetic-only p16 artifact: p16-full-004000."""

ARTIFACT_ID = "p16-full-004000"
SYNTHETIC_ONLY = True
NO_KIO_CLAIM = True
ASSIGNED_FACTS = ['Sakura Clinical, a fictional research organization using a synthetic Windows clinical VDI', 'reference date 2026-09-02', 'synthetic defensive research-administration artifact only', 'no Kio indexing, history, search, chunks, or performance claim']

def record_label() -> str:
    return f"synthetic administration record: {ARTIFACT_ID}"
