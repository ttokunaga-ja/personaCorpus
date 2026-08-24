// Synthetic Product Alpha R7 API contract: API-A7-0375
pub struct ContractSpec {
    pub contract_id: &'static str,
    pub resource: &'static str,
    pub operation: &'static str,
    pub target_p95_ms: u16,
}

pub fn contract_spec() -> ContractSpec {
    ContractSpec { contract_id: "API-A7-0375", resource: "evidence-snapshots", operation: "reconcile", target_p95_ms: 200 }
}
