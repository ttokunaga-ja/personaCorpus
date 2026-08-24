pub const SOURCE_ID: &str = "p01-src-009896";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchitectureEvidence {
    pub review_id: &'static str,
    pub layer: &'static str,
    pub target_ms: u16,
}

pub fn evidence_009896() -> ArchitectureEvidence {
    ArchitectureEvidence { review_id: "ADR-B4-146", layer: "decision journal", target_ms: 225 }
}

pub fn within_target(elapsed_ms: u16) -> bool {
    elapsed_ms <= evidence_009896().target_ms
}
