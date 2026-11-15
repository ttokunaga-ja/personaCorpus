//! Synthetic export parser for EXP-26Q3-007223.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportRecord { pub export_id: &'static str, pub source_id: &'static str, pub generated_at: &'static str, pub latency_ms: u16 }
pub const RECORD: ExportRecord = ExportRecord { export_id: "EXP-26Q3-007223", source_id: "p01-src-007223", generated_at: "2026-08-28T13:41:00+09:00", latency_ms: 225 };
pub fn summary() -> String { format!("{}:{}:{}ms", RECORD.export_id, RECORD.source_id, RECORD.latency_ms) }
#[cfg(test)] mod tests { use super::*; #[test] fn retains_export_identity() { assert!(summary().contains("EXP-26Q3-007223")); } }
