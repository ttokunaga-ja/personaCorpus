/** Synthetic Product Alpha R7 release contract (p01-full-011844). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'released' }> ;

export const record11844: ReleaseEvidence = {
  recordId: 'REL-A7-20260717-11', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'released',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
