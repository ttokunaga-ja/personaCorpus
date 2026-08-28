// Synthetic Orchid Cellworks assay validation for OCW-LAB-XC53GC7B.
const RECORD_CODE: &str = "OCW-LAB-XC53GC7B";

fn accepted(duplicate_cv_percent: f64, recovery_percent: f64) -> bool {
    duplicate_cv_percent <= 10.0 && (90.0..=110.0).contains(&recovery_percent)
}

fn main() {
    let result = accepted(6.1, 93.1);
    println!("{}:OCW-BIO-017:{}", RECORD_CODE, result);
}
