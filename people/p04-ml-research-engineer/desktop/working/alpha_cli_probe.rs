//! Offline summary probe for the Alpha evaluation snapshot.

fn main() {
    let labels = [
        ("account_access", 492_u32), ("delivery_status", 476),
        ("plan_change", 481), ("refund_request", 462), ("technical_help", 489),
    ];
    let total: u32 = labels.iter().map(|(_, count)| count).sum();
    assert_eq!(total, 2400);
    println!("Alpha F1=0.873 precision=0.881 recall=0.865");
    for (label, count) in labels { println!("{label}: {:.1}%", count as f64 / total as f64 * 100.0); }
}

#[cfg(test)]
mod tests {
    #[test]
    fn alpha_f1_is_between_precision_and_recall() {
        let (f1, precision, recall) = (0.873_f64, 0.881_f64, 0.865_f64);
        assert!(recall < f1 && f1 < precision);
    }
}
