// Synthetic ephemeral desktop working copy: p01-src-006174.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-8174", service: "svc-edge-api", latencyMs: 173, state: "review-pending" };
export const workingSource = "p01-src-006174";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
