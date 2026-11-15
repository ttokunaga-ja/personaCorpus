// Synthetic ephemeral desktop working copy: p01-src-008610.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-10610", service: "svc-edge-api", latencyMs: 191, state: "review-pending" };
export const workingSource = "p01-src-008610";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
