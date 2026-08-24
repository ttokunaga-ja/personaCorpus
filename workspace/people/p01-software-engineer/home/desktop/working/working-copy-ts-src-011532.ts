// Synthetic ephemeral desktop working copy: p01-src-011532.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-13532", service: "svc-edge-api", latencyMs: 168, state: "draft" };
export const workingSource = "p01-src-011532";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
