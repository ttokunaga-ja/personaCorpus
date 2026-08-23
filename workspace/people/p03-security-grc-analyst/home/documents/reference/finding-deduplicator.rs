//! Local-only finding-key normalization for audit triage.
use std::collections::BTreeSet;

fn canonical_key(title: &str, control: &str) -> String {
    format!("{}|{}", control.trim().to_uppercase(), title.trim().to_lowercase())
}

fn main() {
    let findings = [
        ("Evidence retention review", "AC-27"),
        ("Evidence retention review", "AC-27"),
        ("Access recertification", "AC-04"),
    ];
    let mut seen = BTreeSet::new();
    for (title, control) in findings {
        let key = canonical_key(title, control);
        if seen.insert(key.clone()) {
            println!("unique,{key}");
        } else {
            println!("duplicate,{key}");
        }
    }
}
