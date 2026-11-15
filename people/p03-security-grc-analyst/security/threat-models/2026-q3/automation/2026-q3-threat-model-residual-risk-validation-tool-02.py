"""Synthetic defensive AC-27 validation helper.
Artifact: p03-full-009099; source: p03-src-009099; review date: 2026-q3; subject: residual-risk.
The second-line assessor confirms the exception has owner, rationale, and closure evidence. The minimized-export boundary excludes raw events and routable operational addresses.
Positive content-spine assertions:
org.cobalt-harbor: Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.\ncontrol.ac-27: Aegis Control AC-27 requires evidence-backed quarterly access review, approval, retention, and exception disposition.\nrel.ac27.evidence-184: Structured, CSV, and log evidence facts precede AC-27 workbooks, review DOCX, searchable PDFs, and security-review decks.\nperiod.2026-q3: Governance planning period from 2026-07-01 through 2026-09-30.\nterm.no-kio-claim: Corpus production and QA establish no Kio indexing, history, chunks, search quality, or performance claim."""

REVIEWED = 48
TIMELY = 47

def validate_p03_full_009099(evidence_items: int, residual_rating: str) -> dict[str, object]:
    """Validate bounded governance facts without accessing systems or networks."""
    completion = evidence_items == 24 and REVIEWED == 48
    return {"artifact_id": "p03-full-009099", "source_id": "p03-src-009099", "review_date": "2026-q3", "subject": "residual-risk", "control": "AC-27", "risk": "RISK-031", "completion": completion, "timely_rate": TIMELY / REVIEWED, "residual_rating": residual_rating}
