//! Metric acceptance rules for Model Alpha M-14 synthetic evaluations.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetricContract {
    pub macro_f1: f64,
    pub precision: f64,
    pub recall: f64,
    pub tolerance: f64,
}

impl MetricContract {
    pub const R184: Self = Self { macro_f1: 0.873, precision: 0.881, recall: 0.865, tolerance: 0.003 };

    pub fn accepts(&self, observed_f1: f64) -> bool {
        (observed_f1 - self.macro_f1).abs() <= self.tolerance
    }

    pub fn display(&self) -> String {
        format!("F1 {:.3}", self.macro_f1)
    }
}

#[cfg(test)]
mod tests {
    use super::MetricContract;

    #[test]
    fn accepts_reference_value() {
        assert!(MetricContract::R184.accepts(0.873));
        assert_eq!(MetricContract::R184.display(), "F1 0.873");
    }
}
