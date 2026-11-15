// Synthetic Product Alpha R7 API contract API-A7-7700
pub struct ApiContract {
    pub contract_id: &'static str,
    pub resource: &'static str,
    pub operation: &'static str,
    pub target_p95_ms: u16,
}

pub fn reviewed_contract() -> ApiContract {
    ApiContract { contract_id: "API-A7-7700", resource: "release-gates", operation: "register", target_p95_ms: 200 }
}
