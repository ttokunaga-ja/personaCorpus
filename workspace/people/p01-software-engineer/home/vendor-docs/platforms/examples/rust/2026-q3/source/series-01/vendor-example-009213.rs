//! Synthetic VND-26Q3-094 evidence adapter for Platform Juniper.
#[derive(Debug)]
pub struct Evidence { pub id: &'static str, pub vendor: &'static str, pub p95_ms: u16, pub coverage_pct: u8 }
pub fn record() -> Evidence { Evidence { id: "VND-26Q3-094", vendor: "Platform Juniper", p95_ms: 187, coverage_pct: 73 } }
pub fn disposition() -> &'static str { "compare implementation notes" }
