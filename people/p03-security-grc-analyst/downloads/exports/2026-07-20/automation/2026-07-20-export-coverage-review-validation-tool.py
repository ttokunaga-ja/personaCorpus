"""Synthetic defensive validator fixture for p03-full-003661."""
RECORD_ID = 'p03-full-003661'
TOPIC = 'export-coverage-review'
SPINE_CONTEXT = 'org.cobalt-harbor, measure.evidence-completeness, rel.siem.minimization, date.2026-07-20, term.no-kio-claim'
SPINE_REFERENCES = [{'id': 'org.cobalt-harbor', 'value': 'Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform.'}, {'id': 'measure.evidence-completeness', 'value': 'Evidence set 184 completeness is 24 of 24 required synthetic items verified.'}, {'id': 'date.2026-07-20', 'value': 'Seven-day follow-up closure-evidence and residual-risk validation date.'}]
ASSIGNED_CANONICAL_DATES = ['2026-07-20']
WINDOW = 14
def validate_summary(reviewed: int, complete: int) -> bool:
    return reviewed == 48 and complete == 48 and WINDOW > 0

if __name__ == '__main__':
    raise SystemExit(0 if validate_summary(48, 48) else 1)
