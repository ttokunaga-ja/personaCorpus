/** Synthetic Product Alpha R7 release contract (p01-full-008794). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'approved' }> ;

export const record8794: ReleaseEvidence = {
  recordId: 'REL-A7-20260715-65', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'approved',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
