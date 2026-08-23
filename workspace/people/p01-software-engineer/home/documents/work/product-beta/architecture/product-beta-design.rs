use std::future::Future;
use std::pin::Pin;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GateState { Pending, Accepted, Rejected }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseGate { pub gate_id: String, pub state: GateState, pub decision_ref: String }
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError { InvalidTransition, MissingDecisionReference }

pub trait GateRepository {
    fn save<'a>(&'a self, gate: ReleaseGate) -> Pin<Box<dyn Future<Output = Result<(), DomainError>> + Send + 'a>>;
}
pub fn decide(mut gate: ReleaseGate, accepted: bool) -> Result<ReleaseGate, DomainError> {
    if gate.state != GateState::Pending { return Err(DomainError::InvalidTransition); }
    if gate.decision_ref != "ADR-042" { return Err(DomainError::MissingDecisionReference); }
    gate.state = if accepted { GateState::Accepted } else { GateState::Rejected };
    Ok(gate)
}
pub const RELEASE_ANCHORS: &str = "Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042";

#[cfg(test)] mod tests { use super::*;
 #[test] fn pending_gate_becomes_terminal_under_adr_042() { let g=ReleaseGate{gate_id:"synthetic-gate-1".into(),state:GateState::Pending,decision_ref:"ADR-042".into()}; assert_eq!(decide(g,true).unwrap().state,GateState::Accepted); }
 #[test] fn terminal_gate_cannot_be_decided_twice() { let g=ReleaseGate{gate_id:"synthetic-gate-2".into(),state:GateState::Accepted,decision_ref:"ADR-042".into()}; assert_eq!(decide(g,false),Err(DomainError::InvalidTransition)); }
}
