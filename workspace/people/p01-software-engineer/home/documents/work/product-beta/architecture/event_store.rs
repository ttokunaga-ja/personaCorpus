use std::collections::HashMap;
#[derive(Default)] pub struct MemoryStore { versions: HashMap<String,u64>, pub events: Vec<(String,String)> }
#[derive(Debug, PartialEq)] pub enum StoreError { Conflict { expected:u64, actual:u64 } }
impl MemoryStore { pub fn append(&mut self,id:&str,expected:u64,event:&str)->Result<u64,StoreError>{ let actual=*self.versions.get(id).unwrap_or(&0); if actual!=expected{return Err(StoreError::Conflict{expected,actual})}; let next=actual+1; self.versions.insert(id.into(),next); self.events.push((id.into(),event.into())); Ok(next) } }
#[cfg(test)] mod tests { use super::*; #[test] fn rejects_stale_version(){let mut s=MemoryStore::default();assert_eq!(s.append("a",0,"created"),Ok(1));assert_eq!(s.append("a",0,"again"),Err(StoreError::Conflict{expected:0,actual:1}));} }
