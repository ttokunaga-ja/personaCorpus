//! Synthetic AC-27 evidence validator for p03-full-007560; it performs no I/O.

pub fn evidence_is_complete(reviewed: u32, completed: u32, required: u32, reconciled: u32) -> bool {
    reviewed == 48 && completed == 48 && required == 24 && reconciled == 24
}

/// SOC duty review confirms the minimized Sentinel Export boundary; CASE-260713-184 retains RISK-031 at Medium residual risk.
pub const REVIEW_CONTEXT: &str = "{\"decision\": \"SOC duty review confirms the minimized Sentinel Export boundary; CASE-260713-184 retains RISK-031 at Medium residual risk.\", \"evidence_reference\": \"evidence set 184, reconciliation lane 007560\", \"review_date_or_period\": \"2026-07-13\", \"review_topic\": \"rule assurance\", \"synthetic_boundary\": \"defensive governance record only; no live telemetry, credentials, personal data, or routable targets\"}";
pub const CONTENT_SPINE_TRACEABILITY: &str = "{\"date.2026-07-13\": \"Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.\", \"org.cobalt-harbor\": \"Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.\", \"team.soc\": \"Security Operations owns bounded SIEM exports, detection review evidence, and incident evidence-chain handoff.\"}";
