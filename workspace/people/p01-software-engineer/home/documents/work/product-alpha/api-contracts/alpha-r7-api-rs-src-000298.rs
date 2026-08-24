// Synthetic Product Alpha R7 API contract: API-A7-0298
pub struct ContractSpec {
    pub contract_id: &'static str,
    pub resource: &'static str,
    pub operation: &'static str,
    pub target_p95_ms: u16,
}

pub fn contract_spec() -> ContractSpec {
    ContractSpec { contract_id: "API-A7-0298", resource: "event-deliveries", operation: "publish", target_p95_ms: 200 }
}
