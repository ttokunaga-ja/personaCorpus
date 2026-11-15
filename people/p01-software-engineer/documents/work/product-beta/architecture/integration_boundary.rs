#[derive(Debug,PartialEq)] pub enum AdapterError { NonTerminal, MissingReference }
pub struct AlphaDecision<'a>{pub gate_id:&'a str,pub terminal:bool,pub decision_ref:Option<&'a str>}
pub fn to_beta_event(input:AlphaDecision<'_>)->Result<String,AdapterError>{if !input.terminal{return Err(AdapterError::NonTerminal)};let adr=input.decision_ref.ok_or(AdapterError::MissingReference)?;Ok(format!("alpha.gate.terminal:{}:{}",input.gate_id,adr))}
