pub const SOURCE_ID: &str = "p01-src-011269";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchitectureEvidence {
    pub review_id: &'static str,
    pub layer: &'static str,
    pub target_ms: u16,
}

pub fn evidence_011269() -> ArchitectureEvidence {
    ArchitectureEvidence { review_id: "ADR-B4-126", layer: "command dispatcher", target_ms: 225 }
}

pub fn within_target(elapsed_ms: u16) -> bool {
    elapsed_ms <= evidence_011269().target_ms
}
