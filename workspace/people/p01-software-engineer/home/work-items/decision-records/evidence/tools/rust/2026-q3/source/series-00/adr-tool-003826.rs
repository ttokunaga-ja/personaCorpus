//! Synthetic decision lifecycle rule for p01-src-003826.
#[derive(Debug, Clone, Copy)]
struct DecisionRecord { record_id: &'static str, adr_id: &'static str, ceiling_ms: u16 }

const RECORD: DecisionRecord = DecisionRecord { record_id: "p01-src-003826", adr_id: "ADR-046", ceiling_ms: 200 };

fn accepted(record: DecisionRecord, observed_p95_ms: u16, status: &str) -> bool {
    matches!(status, "reviewed" | "approved" | "released" | "closed") && observed_p95_ms <= record.ceiling_ms
}

fn main() {
    println!("{} {} {}", RECORD.record_id, RECORD.adr_id, accepted(RECORD, 184, "reviewed"));
}
