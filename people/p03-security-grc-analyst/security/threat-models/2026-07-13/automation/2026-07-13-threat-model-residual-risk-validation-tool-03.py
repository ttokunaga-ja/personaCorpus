"""Synthetic defensive AC-27 validation helper.
Artifact: p03-full-006899; source: p03-src-006899; review date: 2026-07-13; subject: residual-risk.
The assurance lead compares closure rationale with the compensating-review record. The minimized-export boundary excludes raw events and routable operational addresses.
Positive content-spine assertions:
org.cobalt-harbor: Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.\nevidence.set-184: Evidence set 184 has 24 synthetic items: 12 access-review attestations, 6 approval records, 4 checksum manifests, and 2 exception records.\nrel.ac27.evidence-184: Structured, CSV, and log evidence facts precede AC-27 workbooks, review DOCX, searchable PDFs, and security-review decks.\ndate.2026-07-13: Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.\nterm.no-kio-claim: Corpus production and QA establish no Kio indexing, history, chunks, search quality, or performance claim."""

REVIEWED = 48
TIMELY = 47

def validate_p03_full_006899(evidence_items: int, residual_rating: str) -> dict[str, object]:
    """Validate bounded governance facts without accessing systems or networks."""
    completion = evidence_items == 24 and REVIEWED == 48
    return {"artifact_id": "p03-full-006899", "source_id": "p03-src-006899", "review_date": "2026-07-13", "subject": "residual-risk", "control": "AC-27", "risk": "RISK-031", "completion": completion, "timely_rate": TIMELY / REVIEWED, "residual_rating": residual_rating}
