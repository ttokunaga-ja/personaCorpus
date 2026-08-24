/** Synthetic VND-26Q3-083 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation2962: VendorEvaluation = {
  id: 'VND-26Q3-083', vendor: 'Platform Meridian', reviewDate: '2026-08-13', trialP95Ms: 168,
  availabilityPct: 99.81, evidenceCoveragePct: 79, recommendation: 'request evidence clarification'
};

export const meetsPlanningThreshold2962 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
