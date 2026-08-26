//! Content spine traceability: org.cobalt-harbor = Cobalt Harbor Systems | audit.ar-260713-027 = AR-260713-027 is the fictional AC-27 audit request for review material | period.2026-q3 = 2026-Q3 is the governance planning period from 2026-07-01 through 2026-09-30
//! CASE timeline: intake 2026-07-13; normalization 2026-07-14; remediation validation 2026-07-15; follow-up closure 2026-07-20.
//! synthetic defensive governance record; MR-065.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewFact { pub record_id: &'static str, pub control: &'static str, pub case_id: &'static str, pub residual_risk: &'static str }
pub fn validate_review_fact() -> ReviewFact {
    ReviewFact { record_id: "MR-065", control: "AC-27", case_id: "CASE-260713-184", residual_risk: "Medium" }
}
