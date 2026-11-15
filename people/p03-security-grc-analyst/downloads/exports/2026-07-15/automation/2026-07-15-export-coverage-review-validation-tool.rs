// Synthetic defensive validator fixture: p03-full-003621.
const TOPIC: &str = "export-coverage-review";
const SPINE_REFERENCES: &[(&str, &str)] = &[("org.cobalt-harbor", "Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform."), ("system.sentinel-export", "Sentinel Export, the fictional CHS minimized SIEM export service."), ("date.2026-07-15", "Control-owner remediation-plan and retention-disposition validation date.")];
const ASSIGNED_CANONICAL_DATES: &[&str] = &["2026-07-15"];
const EXPORT_WINDOW: u32 = 22;
pub fn completion_rate(reviewed: u32, completed: u32) -> f32 {
    (completed as f32 / reviewed as f32) * 100.0
}
fn main() { assert_eq!(completion_rate(48, 48), 100.0); }
