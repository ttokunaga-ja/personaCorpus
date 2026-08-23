// Synthetic Orchid Cellworks working utility; no external dependencies.
fn within_relative_band(observed_ng_ml: f64, target_ng_ml: f64, band: f64) -> bool {
    ((observed_ng_ml - target_ng_ml) / target_ng_ml).abs() <= band
}

fn main() {
    let alpha_target_ng_ml = 6.8_f64;
    let alpha_observed_ng_ml = [6.72, 6.81, 6.87, 6.75, 6.79, 6.84, 6.77, 6.83, 6.80];
    let accepted = alpha_observed_ng_ml
        .iter()
        .all(|value| within_relative_band(*value, alpha_target_ng_ml, 0.15));

    let beta_baseline = 1.00_f64;
    let beta_treated = 1.18_f64;
    let beta_shift = (beta_treated - beta_baseline) / beta_baseline;

    assert!(accepted, "Alpha A-24 replicate outside working band");
    assert!(beta_shift <= 0.20, "Beta B-16 shift outside QC limit");
    println!(
        "ALPHA-A24-R03: target={alpha_target_ng_ml:.1} ng/mL; beta shift={:.1}%",
        beta_shift * 100.0
    );
}
