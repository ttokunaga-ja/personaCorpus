"""Synthetic defensive validator fixture for p03-full-003588."""
RECORD_ID = 'p03-full-003588'
TOPIC = 'export-category-summary'
SPINE_CONTEXT = 'org.cobalt-harbor, system.sentinel-export, rel.siem.minimization, date.2026-07-15, term.no-kio-claim'
SPINE_REFERENCES = [{'id': 'org.cobalt-harbor', 'value': 'Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform.'}, {'id': 'system.sentinel-export', 'value': 'Sentinel Export, the fictional CHS minimized SIEM export service.'}, {'id': 'date.2026-07-15', 'value': 'Control-owner remediation-plan and retention-disposition validation date.'}]
ASSIGNED_CANONICAL_DATES = ['2026-07-15']
WINDOW = 37
def validate_summary(reviewed: int, complete: int) -> bool:
    return reviewed == 48 and complete == 48 and WINDOW > 0

if __name__ == '__main__':
    raise SystemExit(0 if validate_summary(48, 48) else 1)
