// Synthetic ephemeral desktop working copy: p01-src-011954.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-13954", service: "svc-decision-store", latencyMs: 187, state: "review-pending" };
export const workingSource = "p01-src-011954";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
