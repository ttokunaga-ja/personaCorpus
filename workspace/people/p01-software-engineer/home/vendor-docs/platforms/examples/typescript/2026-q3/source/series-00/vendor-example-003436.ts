/** Synthetic VND-26Q3-077 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation3436: VendorEvaluation = {
  id: 'VND-26Q3-077', vendor: 'Platform Helix', reviewDate: '2026-08-07', trialP95Ms: 152,
  availabilityPct: 99.74, evidenceCoveragePct: 91, recommendation: 'advance to controlled trial'
};

export const meetsPlanningThreshold3436 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
