// Synthetic ephemeral desktop working copy: p01-src-008749.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-10749", service: "svc-release-coordinator", latencyMs: 175, state: "working" };
export const workingSource = "p01-src-008749";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
