// Synthetic ephemeral desktop working copy: p01-src-005774.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-7774", service: "svc-decision-store", latencyMs: 176, state: "review-pending" };
export const workingSource = "p01-src-005774";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
