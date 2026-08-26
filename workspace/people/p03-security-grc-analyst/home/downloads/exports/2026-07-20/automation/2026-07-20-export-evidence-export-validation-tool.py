"""Synthetic defensive validator fixture for p03-full-003626."""
RECORD_ID = 'p03-full-003626'
TOPIC = 'export-evidence-export'
SPINE_CONTEXT = 'org.cobalt-harbor, risk.risk-031, rel.siem.minimization, date.2026-07-20, term.no-kio-claim'
SPINE_REFERENCES = [{'id': 'org.cobalt-harbor', 'value': 'Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform.'}, {'id': 'risk.risk-031', 'value': 'RISK-031: incomplete quarterly privileged-access review evidence; residual Medium after bounded remediation.'}, {'id': 'date.2026-07-20', 'value': 'Seven-day follow-up closure-evidence and residual-risk validation date.'}]
ASSIGNED_CANONICAL_DATES = ['2026-07-20']
WINDOW = 27
def validate_summary(reviewed: int, complete: int) -> bool:
    return reviewed == 48 and complete == 48 and WINDOW > 0

if __name__ == '__main__':
    raise SystemExit(0 if validate_summary(48, 48) else 1)
