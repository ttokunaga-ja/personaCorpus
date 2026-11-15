//! Synthetic S2 aggregate-only check for MTR-09. No I/O and no personal data.

fn main() {
    let daily: [f64; 7] = [10.8, 11.2, 11.5, 12.1, 12.4, 13.0, 13.2];
    let gross = 86.0_f64;
    let recognized_refunds = 1.8_f64;
    let gmv: f64 = daily.iter().sum();
    assert!((gmv - 84.2).abs() < 0.000_001);
    assert!((gross - recognized_refunds - gmv).abs() < 0.000_001);

    let channels = [46.3_f64, 37.9_f64];
    let regions = [26.4_f64, 22.8_f64, 18.6_f64, 16.4_f64];
    let categories = [24.8_f64, 22.1_f64, 19.3_f64, 10.7_f64, 7.3_f64];
    assert!((channels.iter().sum::<f64>() - gmv).abs() < 0.000_001);
    assert!((regions.iter().sum::<f64>() - gmv).abs() < 0.000_001);
    assert!((categories.iter().sum::<f64>() - gmv).abs() < 0.000_001);

    println!("MTR-09 synthetic aggregate check passed: {:.1}M JPY", gmv);
}
