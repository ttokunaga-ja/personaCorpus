// Synthetic Orchid Cellworks assay validation for OCW-LAB-C2EUQKIQ.
const RECORD_CODE: &str = "OCW-LAB-C2EUQKIQ";

fn accepted(duplicate_cv_percent: f64, recovery_percent: f64) -> bool {
    duplicate_cv_percent <= 10.0 && (90.0..=110.0).contains(&recovery_percent)
}

fn main() {
    let result = accepted(2.4, 96.1);
    println!("{}:OCW-BIO-017:{}", RECORD_CODE, result);
}
