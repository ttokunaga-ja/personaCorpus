#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedRecord {
    pub record_id: &'static str,
    pub source_id: &'static str,
    pub project: &'static str,
    pub closed_on: &'static str,
}

pub const SNAPSHOT: ClosedRecord = ClosedRecord {
    record_id: "ARC-26Q3-011325", source_id: "p01-src-011325",
    project: "MIG-26Q3-W03", closed_on: "2026-09-09",
};

pub fn is_closed(record: &ClosedRecord) -> bool { !record.record_id.is_empty() }
