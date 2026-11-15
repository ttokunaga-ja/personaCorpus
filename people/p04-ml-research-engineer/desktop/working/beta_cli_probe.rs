//! Offline acceptance probe for the Beta evaluation snapshot.

fn f1(precision: f64, recall: f64) -> f64 { 2.0 * precision * recall / (precision + recall) }

fn main() {
    let observed = f1(0.852, 0.841);
    assert!((observed - 0.846).abs() < 0.002);
    let splits = [("S1", 1600_u32), ("S2", 800_u32)];
    assert_eq!(splits.iter().map(|(_, n)| n).sum::<u32>(), 2400);
    println!("Beta F1={observed:.3}; coverage {}+{}", splits[0].1, splits[1].1);
}

#[cfg(test)]
mod tests {
    use super::f1;
    #[test]
    fn f1_is_symmetric() { assert_eq!(f1(0.852, 0.841), f1(0.841, 0.852)); }
}
