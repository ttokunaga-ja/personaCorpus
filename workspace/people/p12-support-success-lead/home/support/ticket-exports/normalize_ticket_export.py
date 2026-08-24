#!/usr/bin/env python3
"""Normalize and validate the declared HelioDesk open-ticket JSON schema."""
from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path

REQUIRED = {"ticket_id", "customer", "priority", "channel", "status", "sla_state", "created_at", "sla_due_at", "unresolved_age_minutes", "requester"}
TICKET_ID = re.compile(r"^HD-260804-\d{4}$")


def normalize(path: Path) -> list[dict[str, object]]:
    """Return sorted tickets after checking the documented export shape."""
    payload = json.loads(path.read_text(encoding="utf-8"))
    metadata = payload.get("export_metadata", {})
    if metadata.get("sla_target_hours") != 4 or metadata.get("ticket_count") != 17:
        raise ValueError("expected frozen 17-ticket, four-hour-SLA export")
    tickets = payload.get("tickets")
    if not isinstance(tickets, list) or len(tickets) != 17:
        raise ValueError("tickets must be a list of exactly 17 records")
    ids: set[str] = set()
    for ticket in tickets:
        if not REQUIRED <= ticket.keys():
            raise ValueError(f"missing required field in {ticket}")
        ticket_id = str(ticket["ticket_id"])
        if not TICKET_ID.fullmatch(ticket_id) or ticket_id in ids:
            raise ValueError(f"invalid or duplicate ticket ID: {ticket_id}")
        if not str(ticket["requester"]).startswith("redacted:requester-"):
            raise ValueError(f"requester is not redacted: {ticket_id}")
        ids.add(ticket_id)
    counts = Counter(str(row["priority"]) for row in tickets)
    if counts != Counter(P1=1, P2=5, P3=8, P4=3):
        raise ValueError(f"priority split mismatch: {counts}")
    return sorted(tickets, key=lambda row: str(row["ticket_id"]))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("path", type=Path)
    args = parser.parse_args()
    print(json.dumps(normalize(args.path), indent=2, sort_keys=True))
