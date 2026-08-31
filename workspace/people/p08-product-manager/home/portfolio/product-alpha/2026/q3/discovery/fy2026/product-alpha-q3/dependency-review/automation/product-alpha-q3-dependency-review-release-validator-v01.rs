//! Reproducible synthetic metric utility for p08-full-000867.

const ARTIFACT_ID: &str = "p08-full-000867";
const RUN_ID: &str = "M-27";
const SLICE_F1: [f64; 8] = [0.871, 0.869, 0.865, 0.863, 0.878, 0.857, 0.872, 0.887];

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn main() {
    let minimum = SLICE_F1.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = SLICE_F1.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    println!(
        "{{\"artifact_id\":\"{}\",\"run_id\":\"{}\",\"mean_f1\":{:.3},\"min_f1\":{:.3},\"max_f1\":{:.3}}}",
        ARTIFACT_ID, RUN_ID, mean(&SLICE_F1), minimum, maximum
    );
}
