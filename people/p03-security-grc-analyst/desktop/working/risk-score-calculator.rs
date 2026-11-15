/// Deterministic local scoring for a defensive risk register.
#[derive(Debug)]
struct Risk { likelihood: u8, impact: u8, control_effectiveness: u8 }

fn residual_score(risk: &Risk) -> u8 {
    let inherent = risk.likelihood.saturating_mul(risk.impact);
    inherent.saturating_sub(risk.control_effectiveness.min(inherent))
}

fn band(score: u8) -> &'static str {
    match score { 0..=5 => "low", 6..=12 => "moderate", _ => "high" }
}

fn main() {
    let risk = Risk { likelihood: 4, impact: 5, control_effectiveness: 8 };
    let score = residual_score(&risk);
    println!("RISK-031 residual_score={score} band={}", band(score));
}
