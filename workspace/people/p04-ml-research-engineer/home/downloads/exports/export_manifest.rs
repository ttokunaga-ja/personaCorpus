#[derive(Debug, PartialEq)]
struct Run<'a> {
    name: &'a str,
    precision: f64,
    recall: f64,
    f1: f64,
}

impl Run<'_> {
    fn calculated_f1(&self) -> f64 {
        2.0 * self.precision * self.recall / (self.precision + self.recall)
    }

    fn f1_matches(&self) -> bool {
        (self.calculated_f1() - self.f1).abs() < 0.001
    }
}

fn labels_total(labels: &[(&str, u32)]) -> u32 {
    labels.iter().map(|(_, count)| count).sum()
}

fn main() {
    let labels = [
        ("account_access", 492), ("delivery_status", 476), ("plan_change", 481),
        ("refund_request", 462), ("technical_help", 489),
    ];
    let alpha = Run { name: "Alpha M-14", precision: 0.881, recall: 0.865, f1: 0.873 };
    let beta = Run { name: "Beta B-07", precision: 0.852, recall: 0.841, f1: 0.846 };
    assert_eq!(labels_total(&labels), 2400);
    assert!(alpha.f1_matches() && beta.f1_matches());
    println!("Helix Forge Lab | R184 | HF-Eval-S1/S2 v3.2 | {} examples", labels_total(&labels));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_counts_match_suite_total() {
        assert_eq!(labels_total(&[("account_access", 492), ("delivery_status", 476), ("plan_change", 481), ("refund_request", 462), ("technical_help", 489)]), 2400);
    }

    #[test]
    fn reported_metrics_are_stable() {
        let run = Run { name: "Alpha M-14", precision: 0.881, recall: 0.865, f1: 0.873 };
        assert!(run.f1_matches());
    }
}
