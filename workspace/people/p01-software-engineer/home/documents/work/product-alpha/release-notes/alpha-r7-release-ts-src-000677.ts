/** Synthetic Product Alpha R7 release-note contract: p01-full-000677. */
export type ReleaseNote = Readonly<{ recordId: string; releaseDate: '2026-07-13'; observedP95Ms: 184; status: 'reviewed' }>;

export const releaseNote677: ReleaseNote = {
  recordId: 'REL-A7-20260718-96',
  releaseDate: '2026-07-13',
  observedP95Ms: 184,
  status: 'reviewed',
};

export function isWithinSlo(note: ReleaseNote): boolean { return note.observedP95Ms <= 200; }
