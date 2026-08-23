//! Schema guardrails for the synthetic Helix Forge R184 evaluation bundle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary { pub macro_f1: f32, pub precision: f32, pub recall: f32, pub examples: u32 }

pub fn validate(summary: Summary) -> Result<(), &'static str> {
    if summary.examples != 2400 { return Err("expected S1=1600 plus S2=800"); }
    if !(0.0..=1.0).contains(&summary.macro_f1) || !(0.0..=1.0).contains(&summary.precision) || !(0.0..=1.0).contains(&summary.recall) { return Err("metrics must be probabilities"); }
    Ok(())
}

pub fn alpha_r184() -> Summary { Summary { macro_f1: 0.873, precision: 0.881, recall: 0.865, examples: 2400 } }

#[cfg(test)] mod tests { use super::*; #[test] fn alpha_contract_is_valid() { assert_eq!(validate(alpha_r184()), Ok(())); } }
