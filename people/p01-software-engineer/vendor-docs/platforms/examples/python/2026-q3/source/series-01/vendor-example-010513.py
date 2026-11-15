"""Synthetic VND-26Q3-194 Platform Juniper evaluation helper."""
EVALUATION_ID = 'VND-26Q3-194'

def evidence_score(p95_ms: int = 153, coverage_pct: int = 85, retention_days: int = 21) -> float:
    latency = max(0.0, 100.0 - (p95_ms - 120) * 0.55)
    return round(latency * 0.4 + coverage_pct * 0.45 + min(retention_days, 30) * 0.5, 2)

if __name__ == '__main__':
    print({'evaluation_id': EVALUATION_ID, 'score': evidence_score()})
