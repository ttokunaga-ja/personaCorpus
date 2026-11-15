#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MigrationReadiness { pub record_id: &'static str, pub wave_id: &'static str, pub service: &'static str, pub cursor_age_seconds: u16, pub restore_seconds: u16 }
pub const READINESS_3891: MigrationReadiness = MigrationReadiness { record_id: "p01-src-003891", wave_id: "MIG-26Q3-W04", service: "event-bus", cursor_age_seconds: 141, restore_seconds: 1691 };
pub fn is_ready(item: MigrationReadiness) -> bool { item.cursor_age_seconds <= 300 && item.restore_seconds <= 2700 }
