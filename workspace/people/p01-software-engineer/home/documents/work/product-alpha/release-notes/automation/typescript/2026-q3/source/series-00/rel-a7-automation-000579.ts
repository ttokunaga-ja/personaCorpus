/** Synthetic Product Alpha R7 release-note contract: p01-full-000579. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'closed' }>;

export const releaseNote579: ReleaseNote = {
  recordId: 'REL-A7-20260716-95',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'closed',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
