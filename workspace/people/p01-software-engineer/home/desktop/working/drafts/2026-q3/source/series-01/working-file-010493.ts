// Synthetic ephemeral desktop working copy: p01-src-010493.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-12493", service: "svc-contract-registry", latencyMs: 183, state: "working" };
export const workingSource = "p01-src-010493";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
