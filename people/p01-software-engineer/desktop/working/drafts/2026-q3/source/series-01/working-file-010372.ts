// Synthetic ephemeral desktop working copy: p01-src-010372.
type WorkingSample = { issue: string; service: string; latencyMs: number; state: string };
const sample: WorkingSample = { issue: "ALW-12372", service: "svc-observability", latencyMs: 186, state: "draft" };
export const workingSource = "p01-src-010372";
export const withinAlphaReference = (item: WorkingSample): boolean => item.latencyMs <= 200;
console.log({ ...sample, accepted: false, withinReference: withinAlphaReference(sample) });
