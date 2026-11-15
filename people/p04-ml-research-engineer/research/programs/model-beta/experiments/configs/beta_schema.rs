//! Typed identity for the offline synthetic Model Beta B-07 evaluation.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BetaExperiment {
    pub run_id: &'static str,
    pub model_name: &'static str,
    pub dataset_name: &'static str,
    pub dataset_records: u32,
    pub s1_records: u32,
    pub s2_records: u32,
}

impl BetaExperiment {
    pub const fn r184() -> Self {
        Self { run_id: "R184", model_name: "Model Beta B-07", dataset_name: "HF-Eval-S1/S2 v3.2", dataset_records: 2_400, s1_records: 1_600, s2_records: 800 }
    }

    pub const fn has_reconciled_splits(&self) -> bool {
        self.s1_records + self.s2_records == self.dataset_records
    }
}

#[cfg(test)]
mod tests {
    use super::BetaExperiment;

    #[test]
    fn r184_splits_reconcile() {
        assert!(BetaExperiment::r184().has_reconciled_splits());
    }
}
