"""Working-note focus: 2026 07 13 working notes evidence triage validation tool py s009172; source p03-src-009172 is a bounded synthetic analyst record."""
TRACEABILITY = {'org.cobalt-harbor': 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.', 'date.2026-07-13': 'Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.'}

def review_context() -> dict[str, str]:
    return TRACEABILITY.copy()
