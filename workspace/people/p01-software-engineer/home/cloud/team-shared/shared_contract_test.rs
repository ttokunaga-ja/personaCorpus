/// Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate { Pending, EvidenceAttached, Open, Stopped }

#[allow(dead_code)]
fn transition(from: Gate, checkpoint_matched: bool, p95_ms: u16) -> Gate {
    match from {
        Gate::Pending if checkpoint_matched => Gate::EvidenceAttached,
        Gate::EvidenceAttached if p95_ms <= 200 => Gate::Open,
        Gate::EvidenceAttached => Gate::Stopped,
        state => state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adr_042_opens_only_after_evidence_and_bound() {
        let evidence = transition(Gate::Pending, true, 184);
        assert_eq!(evidence, Gate::EvidenceAttached);
        assert_eq!(transition(evidence, true, 184), Gate::Open);
    }
    #[test]
    fn sustained_latency_stops_transition() {
        assert_eq!(transition(Gate::EvidenceAttached, true, 201), Gate::Stopped);
    }
    #[test]
    fn missing_checkpoint_remains_pending() {
        assert_eq!(transition(Gate::Pending, false, 184), Gate::Pending);
    }
}
