// Synthetic ephemeral desktop working copy: p01-src-005146.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-7146", service: "svc-observability", latencyMs: 168, state: "review-pending" };
export const workingSource = "p01-src-005146";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
