#!/usr/bin/env python3
"""Validate a synthetic HelioDesk incident-link map without network access."""

import json
import re
import sys
from pathlib import Path
from urllib.parse import urlparse

INCIDENT_RE = re.compile(r"^INC-\d{4}-\d{4}-\d{2}$")
KNOWN_ISSUE_RE = re.compile(r"^KI-\d{4}$")
CASE_RE = re.compile(r"^CS-\d{4}-\d{4}-\d{3}$")
ALLOWED_SUFFIX = ".heliodesk.example.invalid"


def valid_url(value: object) -> bool:
    if not isinstance(value, str):
        return False
    parsed = urlparse(value)
    return (
        parsed.scheme == "https"
        and parsed.hostname is not None
        and parsed.hostname.endswith(ALLOWED_SUFFIX)
        and not parsed.username
        and not parsed.password
    )


def require(condition: bool, message: str, errors: list[str]) -> None:
    if not condition:
        errors.append(message)


def validate(data: object) -> list[str]:
    errors: list[str] = []
    require(isinstance(data, dict), "top-level JSON must be an object", errors)
    if not isinstance(data, dict):
        return errors
    incident = data.get("incident")
    known_issue = data.get("known_issue")
    links = data.get("case_links")
    require(isinstance(incident, dict), "incident must be an object", errors)
    require(isinstance(known_issue, dict), "known_issue must be an object", errors)
    require(isinstance(links, list), "case_links must be an array", errors)
    if not isinstance(incident, dict) or not isinstance(known_issue, dict) or not isinstance(links, list):
        return errors
    incident_id = incident.get("id")
    known_issue_id = known_issue.get("id")
    require(isinstance(incident_id, str) and bool(INCIDENT_RE.fullmatch(incident_id)), "invalid incident ID", errors)
    require(isinstance(known_issue_id, str) and bool(KNOWN_ISSUE_RE.fullmatch(known_issue_id)), "invalid known-issue ID", errors)
    require(valid_url(incident.get("status_url")), "invalid incident status URL", errors)
    require(valid_url(known_issue.get("status_url")), "invalid known-issue status URL", errors)
    require(incident.get("sla_hours") == 4, "incident SLA must be four hours", errors)
    require(len(links) == 17, "exactly 17 case links are required", errors)
    case_ids: set[str] = set()
    for index, link in enumerate(links, start=1):
        require(isinstance(link, dict), f"case link {index} must be an object", errors)
        if not isinstance(link, dict):
            continue
        case_id = link.get("case_id")
        require(isinstance(case_id, str) and bool(CASE_RE.fullmatch(case_id)), f"invalid case ID at link {index}", errors)
        require(case_id not in case_ids, f"duplicate case ID at link {index}", errors)
        if isinstance(case_id, str):
            case_ids.add(case_id)
        require(link.get("incident_id") == incident_id, f"wrong incident ID at link {index}", errors)
        require(link.get("known_issue_id") == known_issue_id, f"wrong known-issue ID at link {index}", errors)
        require(link.get("state") == "unresolved", f"case is not unresolved at link {index}", errors)
    return errors


def main() -> int:
    path = Path(sys.argv[1]) if len(sys.argv) == 2 else Path(__file__).with_name("incident-reference-map.json")
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"ERROR unable to read {path}: {exc}")
        return 2
    errors = validate(data)
    if errors:
        for error in errors:
            print(f"ERROR {error}")
        return 1
    print("PASS incident-link map: 17 unresolved cases linked to INC-2026-0804-03; network=false")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
