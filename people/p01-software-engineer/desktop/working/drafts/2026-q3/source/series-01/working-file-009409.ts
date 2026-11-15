// Synthetic ephemeral desktop working copy: p01-src-009409.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-11409", service: "svc-release-coordinator", latencyMs: 184, state: "working" };
export const workingSource = "p01-src-009409";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
