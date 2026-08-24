/** Synthetic Product Alpha R7 release contract (p01-full-006555). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'closed' }> ;

export const record6555: ReleaseEvidence = {
  recordId: 'REL-A7-20260716-57', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'closed',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
