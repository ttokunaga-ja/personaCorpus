// Synthetic Orchid Cellworks assay validation for OCW-LAB-XBS4RKEE.
const RECORD_CODE: &str = "OCW-LAB-XBS4RKEE";

fn accepted(duplicate_cv_percent: f64, recovery_percent: f64) -> bool {
    duplicate_cv_percent <= 10.0 && (90.0..=110.0).contains(&recovery_percent)
}

fn main() {
    let result = accepted(8.2, 92.7);
    println!("{}:OCW-BIO-017:{}", RECORD_CODE, result);
}
