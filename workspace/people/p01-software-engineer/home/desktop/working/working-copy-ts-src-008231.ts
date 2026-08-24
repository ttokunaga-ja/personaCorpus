// Synthetic ephemeral desktop working copy: p01-src-008231.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-10231", service: "svc-contract-registry", latencyMs: 184, state: "revised" };
export const workingSource = "p01-src-008231";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
