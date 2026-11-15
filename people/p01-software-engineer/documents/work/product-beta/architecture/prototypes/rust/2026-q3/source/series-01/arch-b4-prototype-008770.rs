pub const SOURCE_ID: &str = "p01-src-008770";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchitectureEvidence {
    pub review_id: &'static str,
    pub layer: &'static str,
    pub target_ms: u16,
}

pub fn evidence_008770() -> ArchitectureEvidence {
    ArchitectureEvidence { review_id: "ADR-B4-015", layer: "projection reader", target_ms: 225 }
}

pub fn within_target(elapsed_ms: u16) -> bool {
    elapsed_ms <= evidence_008770().target_ms
}
