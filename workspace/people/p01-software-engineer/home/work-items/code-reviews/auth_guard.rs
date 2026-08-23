#[derive(Debug, PartialEq)] pub enum GuardError { MissingContext, Denied, Cancelled }
pub struct Request<'a> { pub role: Option<&'a str>, pub cancelled: bool }
pub fn authorize(request: Request<'_>) -> Result<(), GuardError> {
    if request.cancelled { return Err(GuardError::Cancelled); }
    match request.role { None => Err(GuardError::MissingContext), Some("release-writer") => Ok(()), Some(_) => Err(GuardError::Denied) }
}
#[cfg(test)] mod tests { use super::*;
 #[test] fn cancellation_is_not_mapped_to_denied(){assert_eq!(authorize(Request{role:Some("release-writer"),cancelled:true}),Err(GuardError::Cancelled));}
 #[test] fn missing_context_is_explicit(){assert_eq!(authorize(Request{role:None,cancelled:false}),Err(GuardError::MissingContext));}
}
