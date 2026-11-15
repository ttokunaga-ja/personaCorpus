// Synthetic Cobalt Harbor Systems AC-27 policy evidence model; no network or system access.
pub const ARTIFACT_ID: &str = "p03-full-001462";

pub fn review_summary(label: &str) -> String {
    format!("{}|AC-27|CASE-260713-184|RISK-031|Medium|evidence=184|verified=24|roles=48|complete=48|retain=18m|seq=42", label)
}

// Content-spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | control.retention-18m = Synthetic evidence retention minimum is 18 months; this is a corpus fact, not legal advice. | date.2026-07-20 = Seven-day follow-up validates closure evidence and residual-risk status.
