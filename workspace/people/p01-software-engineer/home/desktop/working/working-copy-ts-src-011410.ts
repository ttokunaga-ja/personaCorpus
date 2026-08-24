// Synthetic ephemeral desktop working copy: p01-src-011410.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-13410", service: "svc-observability", latencyMs: 170, state: "review-pending" };
export const workingSource = "p01-src-011410";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
