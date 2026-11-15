/** Synthetic VND-26Q3-003 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation4562: VendorEvaluation = {
  id: 'VND-26Q3-003', vendor: 'Platform Meridian', reviewDate: '2026-09-05', trialP95Ms: 151,
  availabilityPct: 99.81, evidenceCoveragePct: 83, recommendation: 'request evidence clarification'
};

export const meetsPlanningThreshold4562 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
