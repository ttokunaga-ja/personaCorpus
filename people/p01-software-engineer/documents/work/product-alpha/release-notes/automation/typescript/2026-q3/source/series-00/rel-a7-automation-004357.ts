/** Synthetic Product Alpha R7 release-note contract: p01-full-004357. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'reviewed' }>;

export const releaseNote4357: ReleaseNote = {
  recordId: 'REL-A7-20260718-90',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'reviewed',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
