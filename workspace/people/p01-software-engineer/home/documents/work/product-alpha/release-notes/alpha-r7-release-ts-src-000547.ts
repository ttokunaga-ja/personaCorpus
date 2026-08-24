/** Synthetic Product Alpha R7 release-note contract: p01-full-000547. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'closed' }>;

export const releaseNote547: ReleaseNote = {
  recordId: 'REL-A7-20260716-63',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'closed',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
