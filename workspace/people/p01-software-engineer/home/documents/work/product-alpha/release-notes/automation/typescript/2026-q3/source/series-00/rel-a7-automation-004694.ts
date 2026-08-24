/** Synthetic Product Alpha R7 release-note contract: p01-full-004694. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'approved' }>;

export const releaseNote4694: ReleaseNote = {
  recordId: 'REL-A7-20260719-39',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'approved',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
