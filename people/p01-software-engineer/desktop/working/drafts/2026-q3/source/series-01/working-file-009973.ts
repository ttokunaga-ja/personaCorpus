// Synthetic ephemeral desktop working copy: p01-src-009973.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-11973", service: "svc-release-coordinator", latencyMs: 190, state: "working" };
export const workingSource = "p01-src-009973";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
