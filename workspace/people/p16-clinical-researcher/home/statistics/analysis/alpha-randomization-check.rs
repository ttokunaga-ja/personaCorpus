//! Synthetic Study Alpha 2026 allocation check; no participant data is used.
fn main() {
    let intervention: Vec<f64> = [(-4.4_f64, 12_usize), (-5.2_f64, 12_usize)]
        .iter().copied().flat_map(|(value, count)| std::iter::repeat_n(value, count)).collect();
    let control: Vec<f64> = [(-1.7_f64, 12_usize), (-2.5_f64, 12_usize)]
        .iter().copied().flat_map(|(value, count)| std::iter::repeat_n(value, count)).collect();
    let average = |values: &[f64]| values.iter().sum::<f64>() / values.len() as f64;
    let difference = average(&intervention) - average(&control);
    assert_eq!(intervention.len(), 24);
    assert_eq!(control.len(), 24);
    assert!((average(&intervention) + 4.8).abs() < 1e-9);
    assert!((average(&control) + 2.1).abs() < 1e-9);
    assert!((difference + 2.7).abs() < 1e-9);
    println!("Synthetic allocation: 24/24; Day-28 adjusted difference: {difference:.1}; threshold: 2.5 mg");
}
