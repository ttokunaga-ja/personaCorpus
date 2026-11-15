#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MigrationReadiness { pub record_id: &'static str, pub wave_id: &'static str, pub service: &'static str, pub cursor_age_seconds: u16, pub restore_seconds: u16 }
pub const READINESS_4054: MigrationReadiness = MigrationReadiness { record_id: "p01-src-004054", wave_id: "MIG-26Q3-W05", service: "observability", cursor_age_seconds: 104, restore_seconds: 1854 };
pub fn is_ready(item: MigrationReadiness) -> bool { item.cursor_age_seconds <= 300 && item.restore_seconds <= 2700 }
