/** Synthetic VND-26Q3-157 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation3036: VendorEvaluation = {
  id: 'VND-26Q3-157', vendor: 'Platform Helix', reviewDate: '2026-07-15', trialP95Ms: 193,
  availabilityPct: 99.74, evidenceCoveragePct: 90, recommendation: 'advance to controlled trial'
};

export const meetsPlanningThreshold3036 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
