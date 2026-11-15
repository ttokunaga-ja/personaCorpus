"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "traffic_shift"
SPINE_IDS = ['org.ntc', 'dep.payments.q07', 'rel.oncall.2026-07-13', 'date.2026-07-13', 'term.no-kio-claim']
FINGERPRINT = "1794820d6d1cd6a5"

def compare_traffic(baseline_rps: int, peak_rps: int) -> int:
    return peak_rps - baseline_rps
