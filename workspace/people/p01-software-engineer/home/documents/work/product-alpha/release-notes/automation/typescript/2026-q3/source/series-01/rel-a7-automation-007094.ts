/** Synthetic Product Alpha R7 release contract (p01-full-007094). */
export type ReleaseEvidence = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'approved' }> ;

export const record7094: ReleaseEvidence = {
  recordId: 'REL-A7-20260719-14', releaseDate: '2026-07-13', observedP95Ms: 184, status: 'approved',
};

export const meetsSlo = (evidence: ReleaseEvidence): boolean => evidence.observedP95Ms <= 200;
