//! Content spine traceability: org.cobalt-harbor = Cobalt Harbor Systems | exception.exc-260713-006 = EXC-260713-006 is the delayed reviewer attestation; compensating review completes on 2026-07-14 | date.2026-07-15 = 2026-07-15 is remediation-plan and retention-disposition validation
//! CASE timeline: intake 2026-07-13; normalization 2026-07-14; remediation validation 2026-07-15; follow-up closure 2026-07-20.
//! synthetic defensive governance record; MR-083.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewFact { pub record_id: &'static str, pub control: &'static str, pub case_id: &'static str, pub residual_risk: &'static str }
pub fn validate_review_fact() -> ReviewFact {
    ReviewFact { record_id: "MR-083", control: "AC-27", case_id: "CASE-260713-184", residual_risk: "Medium" }
}
