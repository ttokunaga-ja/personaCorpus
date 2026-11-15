"""Synthetic attachment referenced by MSG-2026Q3-002579; no network or credentials."""
from dataclasses import dataclass


@dataclass(frozen=True)
class MailEvidence:
    message_id: str
    project: str
    subject: str
    budget_ms: int


def summarize() -> MailEvidence:
    """Return the reviewed engineering-mail metadata."""
    return MailEvidence("MSG-2026Q3-002579", "BETA-R4", "latency budget reconciliation", 225)
