// Synthetic Cobalt Harbor Systems AC-27 policy evidence model; no network or system access.
pub const ARTIFACT_ID: &str = "p03-full-007296";

pub fn review_summary(label: &str) -> String {
    format!("{}|AC-27|CASE-260713-184|RISK-031|Medium|evidence=184|verified=24|roles=48|complete=48|retain=18m|seq=233", label)
}

// Content-spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | control.ac-27 = Aegis Control AC-27 requires evidence-backed quarterly access review, approval, retention, and exception disposition. | date.2026-07-15 = Control owner validates the remediation plan and retention disposition.
