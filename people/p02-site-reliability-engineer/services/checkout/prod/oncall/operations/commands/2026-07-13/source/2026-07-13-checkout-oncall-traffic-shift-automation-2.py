"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "traffic_shift"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.oncall.2026-07-13', 'date.2026-07-13', 'term.no-kio-claim']
FINGERPRINT = "645b0bfa2bf8d935"

def compare_traffic(baseline_rps: int, peak_rps: int) -> int:
    return peak_rps - baseline_rps
