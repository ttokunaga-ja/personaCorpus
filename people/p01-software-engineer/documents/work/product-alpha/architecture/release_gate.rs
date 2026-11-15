// Aster Loop Product Alpha R7 2026-07-13 p95 184 ms ADR-042
use std::collections::HashMap;
#[derive(Clone,Debug,PartialEq,Eq)]pub enum State{Admitted,Decided(bool),TimedOut}
#[derive(Debug,PartialEq)]pub enum Error{Illegal}
#[derive(Default)]pub struct Gate{r:HashMap<String,State>}
impl Gate{pub fn admit(&mut self,k:&str)->State{self.r.entry(k.into()).or_insert(State::Admitted).clone()}pub fn decide(&mut self,k:&str,a:bool)->Result<(),Error>{match self.r.get(k){Some(State::Admitted)=>{self.r.insert(k.into(),State::Decided(a));Ok(())},Some(State::Decided(x))if*x==a=>Ok(()),_=>Err(Error::Illegal)}}pub fn timeout(&mut self,k:&str)->Result<(),Error>{match self.r.get(k){Some(State::Admitted)=>{self.r.insert(k.into(),State::TimedOut);Ok(())},_=>Err(Error::Illegal)}}}
#[cfg(test)]mod tests{use super::*;#[test]fn idem(){let mut g=Gate::default();assert_eq!(g.admit("r7"),g.admit("r7"));}#[test]fn terminal(){let mut g=Gate::default();g.admit("r7");g.timeout("r7").unwrap();assert_eq!(g.decide("r7",true),Err(Error::Illegal));}}
