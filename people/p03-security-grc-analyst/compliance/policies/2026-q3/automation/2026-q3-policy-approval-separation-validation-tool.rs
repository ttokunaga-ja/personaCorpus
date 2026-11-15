// Synthetic Cobalt Harbor Systems AC-27 policy evidence model; no network or system access.
pub const ARTIFACT_ID: &str = "p03-full-001535";

pub fn review_summary(label: &str) -> String {
    format!("{}|AC-27|CASE-260713-184|RISK-031|Medium|evidence=184|verified=24|roles=48|complete=48|retain=18m|seq=56", label)
}

// Content-spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | decision.dec-260713-004 = Retain evidence set 184 under the synthetic GRC retention schedule; no live-source data is included. | period.2026-q3 = Governance planning period from 2026-07-01 through 2026-09-30.
