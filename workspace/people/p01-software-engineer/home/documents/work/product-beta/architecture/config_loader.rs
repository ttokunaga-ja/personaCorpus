#[derive(Debug,PartialEq)] pub struct Config { pub retries:u8, pub checkpoint_batch:usize, pub fixture:bool }
pub fn load(env:Option<(u8,usize)>, fixture:bool)->Result<Config,String>{let(retries,batch)=env.unwrap_or((3,50));if batch==0{return Err("checkpoint_batch must be positive".into())};Ok(Config{retries,checkpoint_batch:batch,fixture})}
#[cfg(test)] mod tests {use super::*;#[test]fn validates_synthetic_fixture(){assert_eq!(load(Some((2,0)),true).unwrap_err(),"checkpoint_batch must be positive");}}
