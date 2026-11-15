// Synthetic ephemeral desktop working copy: p01-src-006891.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-8891", service: "svc-event-bus", latencyMs: 177, state: "revised" };
export const workingSource = "p01-src-006891";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
