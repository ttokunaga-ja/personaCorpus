// Synthetic ephemeral desktop working copy: p01-src-005708.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-7708", service: "svc-decision-store", latencyMs: 172, state: "draft" };
export const workingSource = "p01-src-005708";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
