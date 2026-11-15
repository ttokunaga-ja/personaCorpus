#[derive(Clone,Copy,Debug,Eq,PartialEq)] pub enum Flag { AsyncProjection, AlphaR7Adapter }
#[derive(Clone,Copy,Debug)] pub struct Definition { pub flag:Flag, pub default:bool, pub expiry:&'static str }
pub const DEFINITIONS:[Definition;2]=[Definition{flag:Flag::AsyncProjection,default:false,expiry:"2026-10-13"},Definition{flag:Flag::AlphaR7Adapter,default:false,expiry:"2026-10-13"}];
pub fn audit(flag:Flag, enabled:bool)->String{format!("synthetic audit flag={flag:?} enabled={enabled}")}
