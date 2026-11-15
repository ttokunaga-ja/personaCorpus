// Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
pub const TOPIC: &str = "node_pressure";
pub const FINGERPRINT: &str = "5fc51de127066661";
pub fn headroom_ok(percent: u8) -> bool { percent >= 31 }
