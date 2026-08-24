/** Synthetic platform evidence contract for VND-26Q3-103. */
export type PlatformEvidence = { id: string; vendor: string; reviewDate: string; p95Ms: number; coveragePct: number; disposition: string };
export const evidence8262: PlatformEvidence = { id: 'VND-26Q3-103', vendor: 'Platform Meridian', reviewDate: '2026-07-14', p95Ms: 164, coveragePct: 74, disposition: 'schedule protocol walkthrough' };
export const isReady8262 = (item: PlatformEvidence) => item.p95Ms <= 200 && item.coveragePct >= 75;
