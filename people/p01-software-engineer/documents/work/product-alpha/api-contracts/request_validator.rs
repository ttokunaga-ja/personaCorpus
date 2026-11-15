#[derive(Debug, PartialEq)] pub struct ApiError { pub code: &'static str, pub status: u16, pub retryable: bool }
pub fn validate_create(version: &str, summary: &str, key: Option<&str>) -> Result<(), ApiError> {
    if key.map_or(true, |v| v.len() < 8 || v.len() > 128) { return Err(ApiError { code: "idempotency_key_required", status: 400, retryable: false }); }
    if !version.starts_with("r7.") { return Err(ApiError { code: "invalid_version", status: 422, retryable: false }); }
    if summary.trim().is_empty() || summary.len() > 500 { return Err(ApiError { code: "validation_failed", status: 422, retryable: false }); }
    Ok(())
}
pub fn validate_response(request_id: &str, state: &str) -> Result<(), ApiError> {
    let valid = ["requested", "awaiting_approval", "approved", "running", "completed", "failed"];
    if request_id.is_empty() || !valid.contains(&state) { return Err(ApiError { code: "contract_violation", status: 500, retryable: true }); } Ok(())
}
#[cfg(test)] mod tests { use super::*; #[test] fn accepts_r7_gate_request() { assert_eq!(validate_create("r7.0", "ADR-042 gate", Some("12345678")), Ok(())); } #[test] fn rejects_missing_key() { assert_eq!(validate_create("r7.0", "x", None).unwrap_err().status, 400); } #[test] fn rejects_old_version() { assert_eq!(validate_create("r6.9", "x", Some("12345678")).unwrap_err().code, "invalid_version"); } }
