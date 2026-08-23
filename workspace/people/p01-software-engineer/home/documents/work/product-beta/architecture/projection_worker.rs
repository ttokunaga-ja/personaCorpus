#[derive(Default, Debug, PartialEq)] pub struct Checkpoint { pub offset:u64 }
pub trait Projection { fn apply(&mut self, event_id:&str)->Result<(),String>; }
pub fn run_once(p:&mut impl Projection, checkpoint:&mut Checkpoint, event_id:&str, offset:u64)->Result<(),String>{ if offset<=checkpoint.offset{return Ok(())}; p.apply(event_id)?; checkpoint.offset=offset; Ok(()) }
pub fn retry(mut attempts:u8, mut job:impl FnMut()->Result<(),String>)->Result<(),String>{loop{match job(){Ok(())=>return Ok(()),Err(_e) if attempts>0=>attempts-=1,Err(e)=>return Err(e)}}}
