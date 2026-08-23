//! Typed schema fragments for the offline synthetic R184 configuration family.

#[derive(Debug, Clone, PartialEq)]
pub struct ExperimentIdentity {
    pub run_id: &'static str,
    pub model_name: &'static str,
    pub dataset_name: &'static str,
    pub dataset_records: u32,
}

impl ExperimentIdentity {
    pub const fn r184() -> Self {
        Self {
            run_id: "R184",
            model_name: "Model Alpha M-14",
            dataset_name: "HF-Eval-S1/S2 v3.2",
            dataset_records: 2_400,
        }
    }

    pub fn is_synthetic_scope(&self) -> bool {
        self.dataset_records == 2_400 && self.run_id == "R184"
    }
}

#[cfg(test)]
mod tests {
    use super::ExperimentIdentity;

    #[test]
    fn r184_is_a_fixed_synthetic_identity() {
        assert!(ExperimentIdentity::r184().is_synthetic_scope());
    }
}
