// Synthetic AC-27 validation helper for p03-full-008127; no network or file access.
// stage=normalization; card=EV-184-16; lane=PRIV-16; pass=10; working_paper=8127; spine=org.cobalt-harbor, evidence.set-184, rel.ac27.evidence-184, date.2026-07-14, term.no-kio-claim
// Checksum normalization and action-owner acknowledgement were reviewed without accessing a live system. Evidence indexing links bounded evidence references to the AC-27 review and retention disposition. Evidence set 184 contains 24 synthetic items: 12 access-review attestations, 6 approval records, 4 checksum manifests, and 2 exception records.
const ORGANIZATION: &str = "Cobalt Harbor Systems";
pub fn valid(control_id: &str, case_id: &str, risk_id: &str, evidence_set: u32, reviewed: u32, timely: u32, residual: &str) -> bool {
    let _organization = ORGANIZATION;
    control_id == "AC-27" && case_id == "CASE-260713-184" && risk_id == "RISK-031" && evidence_set == 184 && reviewed == 48 && timely == 47 && residual == "Medium"
}
