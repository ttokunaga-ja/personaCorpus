// Synthetic ephemeral desktop working copy: p01-src-008349.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-10349", service: "svc-event-bus", latencyMs: 178, state: "working" };
export const workingSource = "p01-src-008349";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
