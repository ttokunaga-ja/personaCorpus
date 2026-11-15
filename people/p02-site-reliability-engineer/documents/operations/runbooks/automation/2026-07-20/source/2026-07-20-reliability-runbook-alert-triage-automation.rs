// Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
pub const TOPIC: &str = "alert_triage";
pub const FINGERPRINT: &str = "1e51d242705be563";
pub fn q07_is_critical(depth: u32) -> bool { depth >= 24_000 }
