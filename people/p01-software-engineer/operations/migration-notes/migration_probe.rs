#[derive(Debug, PartialEq)] pub struct ProbeInput { pub compatible: bool, pub checkpoint_lag_s: u64, pub queue_lag_s: u64, pub p95_ms: u64, pub decision_ref: String }
#[derive(Debug, PartialEq)] pub enum Finding { Compatible, CheckpointLag, QueueLag, Latency, MissingDecisionRef }
pub fn inspect(input: &ProbeInput) -> Vec<Finding> {
 let mut findings=Vec::new(); if !input.compatible { findings.push(Finding::Compatible); }
 if input.checkpoint_lag_s>30 { findings.push(Finding::CheckpointLag); }
 if input.queue_lag_s>60 { findings.push(Finding::QueueLag); }
 if input.p95_ms>200 { findings.push(Finding::Latency); }
 if input.decision_ref!="ADR-042" { findings.push(Finding::MissingDecisionRef); } findings
}
pub fn ready(input:&ProbeInput)->bool { inspect(input).is_empty() }
#[cfg(test)] mod tests { use super::*;
 fn healthy()->ProbeInput{ProbeInput{compatible:true,checkpoint_lag_s:9,queue_lag_s:14,p95_ms:184,decision_ref:"ADR-042".into()}}
 #[test] fn accepts_r7_reference_observation(){assert!(ready(&healthy()));}
 #[test] fn reports_multiple_stop_conditions(){let mut p=healthy();p.queue_lag_s=61;p.decision_ref="missing".into();assert_eq!(inspect(&p),vec![Finding::QueueLag,Finding::MissingDecisionRef]);}
}
