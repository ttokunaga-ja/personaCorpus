// Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
pub const TOPIC: &str = "log_gap";
pub const FINGERPRINT: &str = "52845de30aa56826";
pub fn export_gap_reconciled(minutes: u8) -> bool { minutes == 7 }
