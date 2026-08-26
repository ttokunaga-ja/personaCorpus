"""Working-note focus: 2026 07 13 working notes timeline annotation validation tool py s006129; source p03-src-006129 is a bounded synthetic analyst record."""
TRACEABILITY = {'org.cobalt-harbor': 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.', 'case.case-260713-184': 'CASE-260713-184 is the synthetic evidence intake case for evidence set 184, not a real incident.', 'date.2026-07-13': 'Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.'}

def review_context() -> dict[str, str]:
    return TRACEABILITY.copy()
