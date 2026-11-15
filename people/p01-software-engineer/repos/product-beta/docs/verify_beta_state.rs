//! Aster Loop / Product Alpha R7 / 2026-07-13 / p95 184 ms / ADR-042
#[derive(Debug, Clone, PartialEq, Eq)] pub struct BetaState { pub alpha_checkpoint:u64, pub beta_checkpoint:u64, pub queue_lag_seconds:u64, pub latency_p95_ms:u64, pub alpha_records:u64, pub beta_records:u64 }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct Finding { pub invariant:&'static str, pub expected:String, pub observed:String }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct Verification { pub ok:bool, pub findings:Vec<Finding> }
pub fn verify(s:&BetaState)->Verification { let mut f=Vec::new();
 if s.alpha_checkpoint!=s.beta_checkpoint { f.push(Finding{invariant:"checkpoint",expected:s.alpha_checkpoint.to_string(),observed:s.beta_checkpoint.to_string()});}
 if s.queue_lag_seconds>300 { f.push(Finding{invariant:"queue_lag",expected:"<=300".into(),observed:s.queue_lag_seconds.to_string()});}
 if s.latency_p95_ms>184 { f.push(Finding{invariant:"p95_latency",expected:"<=184".into(),observed:s.latency_p95_ms.to_string()});}
 if s.alpha_records!=s.beta_records { f.push(Finding{invariant:"record_count",expected:s.alpha_records.to_string(),observed:s.beta_records.to_string()});}
 Verification{ok:f.is_empty(),findings:f} }
#[cfg(test)] mod tests { use super::*; fn base()->BetaState{BetaState{alpha_checkpoint:4,beta_checkpoint:4,queue_lag_seconds:3,latency_p95_ms:184,alpha_records:12,beta_records:12}}
 #[test]fn clean_state_passes(){assert!(verify(&base()).ok)} #[test]fn detects_checkpoint(){let mut s=base();s.beta_checkpoint=3;assert_eq!(verify(&s).findings[0].invariant,"checkpoint")} #[test]fn detects_lag(){let mut s=base();s.queue_lag_seconds=301;assert!(!verify(&s).ok)} #[test]fn detects_latency_and_counts(){let mut s=base();s.latency_p95_ms=185;s.beta_records=11;assert_eq!(verify(&s).findings.len(),2)} }
