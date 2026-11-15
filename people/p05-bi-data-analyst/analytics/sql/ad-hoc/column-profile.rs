/// Profiles synthetic aggregate columns without retaining record-level data.
fn profile(values: &[i64]) -> (usize, i64, i64, i64) {
    assert!(!values.is_empty());
    let total: i64 = values.iter().sum();
    let minimum = *values.iter().min().unwrap();
    let maximum = *values.iter().max().unwrap();
    (values.len(), minimum, maximum, total)
}

fn main() {
    let result = profile(&[46_300_000, 37_900_000]);
    assert_eq!(result, (2, 37_900_000, 46_300_000, 84_200_000));
    println!("count={} min={} max={} total={}", result.0, result.1, result.2, result.3);
}
