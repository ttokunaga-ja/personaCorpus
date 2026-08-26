// Cobalt Harbor Systems Sentinel Export; evidence set 184; EV-184-07; export minimization; 2026-07-20; synthetic minimized defensive SIEM context. Spine anchors: org.cobalt-harbor, evidence.ev-184-07, rel.siem.minimization, date.2026-07-20, term.no-kio-claim.
pub const ARTIFACT_ID: &str = "p03-full-002401";
pub const SOURCE_ID: &str = "p03-src-002401";
pub const RECORD_SEQUENCE: i64 = 2401;
pub fn valid_minimized_count(category: &str, count: i64) -> bool {
    matches!(category, "access-review" | "approval-record" | "checksum-reconciliation") && count >= 0
}
