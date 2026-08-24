/** Synthetic Product Alpha R7 release contract (p01-full-009280). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'released' }> ;

export const record9280: ReleaseEvidence = {
  recordId: 'REL-A7-20260713-66', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'released',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
