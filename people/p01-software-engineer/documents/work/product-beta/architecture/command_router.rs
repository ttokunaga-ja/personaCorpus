use std::future::Future; use std::pin::Pin;
#[derive(Debug, PartialEq)] pub enum RouterError { Cancelled, InvalidCommand, Dependency(String) }
pub struct Request { pub kind: String, pub cancelled: bool }
pub trait Handler { fn handle<'a>(&'a self, kind: &'a str) -> Pin<Box<dyn Future<Output=Result<String, RouterError>> + Send + 'a>>; }
pub async fn dispatch(handler: &impl Handler, request: Request) -> Result<String, RouterError> {
    if request.cancelled { return Err(RouterError::Cancelled); }
    if request.kind.trim().is_empty() { return Err(RouterError::InvalidCommand); }
    handler.handle(&request.kind).await
}
