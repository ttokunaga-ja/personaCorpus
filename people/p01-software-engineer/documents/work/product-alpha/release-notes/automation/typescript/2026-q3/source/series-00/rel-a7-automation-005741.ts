/** Synthetic Product Alpha R7 release contract (p01-full-005741). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'reviewed' }> ;

export const record5741: ReleaseEvidence = {
  recordId: 'REL-A7-20260718-19', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'reviewed',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
