use std::collections::BTreeSet;

fn sanitize_field(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn allow_label(label: &str) -> bool {
    let labels = BTreeSet::from([
        "account_access", "delivery_status", "plan_change", "refund_request", "technical_help",
    ]);
    labels.contains(label)
}

fn main() {
    let raw = "  HF-Eval-S1/S2\n  v3.2 ";
    let cleaned = sanitize_field(raw);
    assert_eq!(cleaned, "HF-Eval-S1/S2 v3.2");
    assert!(allow_label("technical_help"));
    println!("sanitized export metadata: {cleaned}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_spacing_and_controls() {
        assert_eq!(sanitize_field("\tAlpha\u{0007} M-14\n"), "Alpha M-14");
    }

    #[test]
    fn only_known_labels_are_allowed() {
        assert!(allow_label("refund_request"));
        assert!(!allow_label("unclassified"));
    }
}
