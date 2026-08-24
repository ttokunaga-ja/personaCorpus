//! Synthetic VND-26Q3-166 evaluation model for Platform Juniper.
#[derive(Debug, Clone)]
pub struct VendorEvaluation { pub id: &'static str, pub vendor: &'static str, pub p95_ms: u16, pub availability_pct: f32, pub coverage_pct: u8 }

pub fn evaluation() -> VendorEvaluation {
    VendorEvaluation { id: "VND-26Q3-166", vendor: "Platform Juniper", p95_ms: 178, availability_pct: 99.87, coverage_pct: 96 }
}

pub fn review_note() -> &'static str { "retain for comparative review" }
