"""Synthetic-only p16 artifact: p16-full-002501."""

ARTIFACT_ID = "p16-full-002501"
SYNTHETIC_ONLY = True
NO_KIO_CLAIM = True
ASSIGNED_FACTS = ['Sakura Clinical, a fictional research organization using a synthetic Windows clinical VDI', 'reference date 2026-09-02', 'fictional review threshold 2.5 mg (not a treatment recommendation)', 'synthetic defensive research-administration artifact only', 'no Kio indexing, history, search, chunks, or performance claim']

def record_label() -> str:
    return f"synthetic administration record: {ARTIFACT_ID}"
