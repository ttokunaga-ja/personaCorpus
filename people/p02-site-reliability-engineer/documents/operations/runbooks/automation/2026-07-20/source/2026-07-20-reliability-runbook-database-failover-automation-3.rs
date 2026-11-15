// Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
pub const TOPIC: &str = "database_failover";
pub const FINGERPRINT: &str = "b62c4ced51b15dcb";
pub fn failover_evidence_reviewed(status: &str) -> bool { status == "reviewed" }
