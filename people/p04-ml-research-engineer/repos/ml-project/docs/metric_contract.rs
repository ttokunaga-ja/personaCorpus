//! Minimal metric contract for synthetic evaluation documentation.

#[derive(Debug, Clone, Copy)]
struct MetricSummary {
    precision: f64,
    recall: f64,
    macro_f1: f64,
}

impl MetricSummary {
    fn is_consistent(self) -> bool {
        let harmonic = 2.0 * self.precision * self.recall / (self.precision + self.recall);
        (harmonic - self.macro_f1).abs() < 0.001
    }
}

fn main() {
    let alpha = MetricSummary { precision: 0.881, recall: 0.865, macro_f1: 0.873 };
    let beta = MetricSummary { precision: 0.852, recall: 0.841, macro_f1: 0.846 };
    assert!(alpha.is_consistent());
    assert!(beta.is_consistent());
    assert!(alpha.macro_f1 > beta.macro_f1);
    println!("macro-F1 contract accepted: alpha {:.3}, beta {:.3}", alpha.macro_f1, beta.macro_f1);
}

#[cfg(test)]
mod tests {
    use super::MetricSummary;

    #[test]
    fn accepts_the_documented_alpha_metrics() {
        let alpha = MetricSummary { precision: 0.881, recall: 0.865, macro_f1: 0.873 };
        assert!(alpha.is_consistent());
    }
}
