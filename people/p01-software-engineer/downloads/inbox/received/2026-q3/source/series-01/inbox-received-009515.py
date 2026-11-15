"""Synthetic incoming ALPHA-R7 material; status remains inbox/unreviewed."""
from dataclasses import dataclass

@dataclass(frozen=True)
class InboxMaterial:
    event: str
    source_id: str
    review_status: str
    observed_p95_ms: int

MATERIAL = InboxMaterial(
    event="INBOX-009515",
    source_id="p01-src-009515",
    review_status="unreviewed-inbox",
    observed_p95_ms=184,
)

def intake_summary(material: InboxMaterial = MATERIAL) -> dict[str, object]:
    return {"release": "ALPHA-R7", "event": material.event, "status": material.review_status, "accepted": False}
