/** Synthetic VND-26Q3-177 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation4736: VendorEvaluation = {
  id: 'VND-26Q3-177', vendor: 'Platform Helix', reviewDate: '2026-09-11', trialP95Ms: 178,
  availabilityPct: 99.74, evidenceCoveragePct: 89, recommendation: 'advance to controlled trial'
};

export const meetsPlanningThreshold4736 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
