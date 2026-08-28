// Synthetic Orchid Cellworks assay validation for OCW-LAB-2TD2Q7Y6.
const RECORD_CODE: &str = "OCW-LAB-2TD2Q7Y6";

fn accepted(duplicate_cv_percent: f64, recovery_percent: f64) -> bool {
    duplicate_cv_percent <= 10.0 && (90.0..=110.0).contains(&recovery_percent)
}

fn main() {
    let result = accepted(5.0, 104.7);
    println!("{}:OCW-BIO-017:{}", RECORD_CODE, result);
}
