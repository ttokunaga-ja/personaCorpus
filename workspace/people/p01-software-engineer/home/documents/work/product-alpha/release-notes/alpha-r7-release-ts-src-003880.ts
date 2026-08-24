/** Synthetic Product Alpha R7 release-note contract: p01-full-003880. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'released' }>;

export const releaseNote3880: ReleaseNote = {
  recordId: 'REL-A7-20260713-01',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'released',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
