"""Synthetic attachment referenced by MSG-2026Q3-006659; no network or credentials."""
from dataclasses import dataclass


@dataclass(frozen=True)
class MailEvidence:
    message_id: str
    project: str
    subject: str
    budget_ms: int


def summarize() -> MailEvidence:
    """Return the reviewed engineering-mail metadata."""
    return MailEvidence("MSG-2026Q3-006659", "BETA-R4", "migration wave handoff", 225)
