//! Metric checks for the synthetic Model Beta B-07 candidate.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BetaMetrics {
    pub macro_f1: f64,
    pub precision: f64,
    pub recall: f64,
}

impl BetaMetrics {
    pub const R184: Self = Self { macro_f1: 0.846, precision: 0.852, recall: 0.841 };

    pub fn trails_reference(&self, reference_f1: f64) -> bool {
        self.macro_f1 < reference_f1
    }

    pub fn display(&self) -> String {
        format!("F1 {:.3}", self.macro_f1)
    }
}

#[cfg(test)]
mod tests {
    use super::BetaMetrics;

    #[test]
    fn candidate_is_below_alpha_reference() {
        assert!(BetaMetrics::R184.trails_reference(0.873));
        assert_eq!(BetaMetrics::R184.display(), "F1 0.846");
    }
}
