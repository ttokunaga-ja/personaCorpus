/** Synthetic Product Alpha R7 release contract (p01-full-006141). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'reviewed' }> ;

export const record6141: ReleaseEvidence = {
  recordId: 'REL-A7-20260718-31', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'reviewed',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
