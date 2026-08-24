/** Synthetic Product Alpha R7 release contract (p01-full-010633). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'reviewed' }> ;

export const record10633: ReleaseEvidence = {
  recordId: 'REL-A7-20260714-61', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'reviewed',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
