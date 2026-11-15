//! Aster Loop Product Alpha R7 2026-07-13 p95 184 ms ADR-042
use std::collections::HashMap;
pub fn admit(m:&mut HashMap<String,String>, id:&str)->String { m.entry(id.into()).or_insert_with(||"admitted".into()).clone() }
#[cfg(test)] mod tests { use super::*; #[test] fn duplicate_event_returns_original(){let mut m=HashMap::new();assert_eq!(admit(&mut m,"r7"),"admitted");m.insert("r7".into(),"decided".into());assert_eq!(admit(&mut m,"r7"),"decided");} }
