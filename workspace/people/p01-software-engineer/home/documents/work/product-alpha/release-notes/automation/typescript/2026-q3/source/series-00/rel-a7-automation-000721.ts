/** Synthetic Product Alpha R7 release-note contract: p01-full-000721. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'reviewed' }>;

export const releaseNote721: ReleaseNote = {
  recordId: 'REL-A7-20260714-43',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'reviewed',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
