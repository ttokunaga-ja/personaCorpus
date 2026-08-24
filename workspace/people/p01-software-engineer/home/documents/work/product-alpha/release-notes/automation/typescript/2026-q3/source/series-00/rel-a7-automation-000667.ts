/** Synthetic Product Alpha R7 release-note contract: p01-full-000667. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'closed' }>;

export const releaseNote667: ReleaseNote = {
  recordId: 'REL-A7-20260716-86',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'closed',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
