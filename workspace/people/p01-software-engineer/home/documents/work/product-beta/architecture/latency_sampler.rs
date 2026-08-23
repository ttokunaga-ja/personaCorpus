pub fn percentile_ms(sorted: &[u64], percentile: f64) -> Option<u64> { if sorted.is_empty() || !(0.0..=1.0).contains(&percentile){return None}; let index=((sorted.len()-1) as f64*percentile).ceil() as usize; sorted.get(index).copied() }
pub fn r7_reference_p95() -> u64 { percentile_ms(&[91,176,179,181,184],0.95).unwrap() }
#[cfg(test)] mod tests { use super::*; #[test] fn captures_synthetic_184_ms_p95(){assert_eq!(r7_reference_p95(),184)} }
