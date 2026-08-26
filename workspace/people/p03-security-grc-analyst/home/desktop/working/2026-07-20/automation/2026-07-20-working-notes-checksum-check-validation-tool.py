"""Working-note focus: 2026 07 20 working notes checksum check validation tool py s003102; source p03-src-003102 is a bounded synthetic analyst record."""
TRACEABILITY = {'org.cobalt-harbor': 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.', 'case.case-260713-184': 'CASE-260713-184 is the synthetic evidence intake case for evidence set 184, not a real incident.', 'date.2026-07-20': 'Seven-day follow-up validates closure evidence and residual-risk status.'}

def review_context() -> dict[str, str]:
    return TRACEABILITY.copy()
