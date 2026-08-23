// Product Alpha R7 2026-07-13 p95 184 ms ADR-042
fn submit(id:&str,corr:&str)->Result<String,&'static str>{if id.is_empty()||corr.is_empty(){Err("missing correlation")}else{Ok(format!("admitted:{id}:{corr}"))}}fn main(){match submit("r7-demo","corr-demo"){Ok(x)=>println!("{x}"),Err(e)=>eprintln!("{e}")}}
