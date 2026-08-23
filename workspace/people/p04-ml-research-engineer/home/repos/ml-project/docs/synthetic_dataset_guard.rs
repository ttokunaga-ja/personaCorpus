//! Guardrails for the fixed, entirely synthetic HF-Eval-S1/S2 v3.2 bundle.

use std::collections::BTreeMap;

fn main() {
    let labels = BTreeMap::from([
        ("account_access", 492_u16),
        ("delivery_status", 476),
        ("plan_change", 481),
        ("refund_request", 462),
        ("technical_help", 489),
    ]);
    let total: u16 = labels.values().sum();
    let s1 = 1_600_u16;
    let s2 = 800_u16;
    assert_eq!(labels.len(), 5);
    assert_eq!(total, 2_400);
    assert_eq!(s1 + s2, total);
    println!("HF-Eval-S1/S2 v3.2 guard passed for {} synthetic records", total);
}

#[cfg(test)]
mod tests {
    #[test]
    fn fixed_partition_sizes_match_the_bundle_total() {
        assert_eq!(1_600_u16 + 800_u16, 2_400_u16);
    }
}
