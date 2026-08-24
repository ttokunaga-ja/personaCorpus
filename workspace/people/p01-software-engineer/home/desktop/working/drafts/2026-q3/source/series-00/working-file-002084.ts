// Ephemeral desktop working copy for p01-src-002084; synthetic only.
type WorkingObservation = { issueId: string; service: string; latencyMs: number; state: "draft" | "working" | "review-pending" | "revised" };

const observation: WorkingObservation = { issueId: "ALW-3084", service: "svc-decision-store", latencyMs: 176, state: "draft" };

export const isWithinAlphaReference = (item: WorkingObservation): boolean => item.latencyMs <= 200;
export const sourceId = "p01-src-002084";
console.log({ sourceId, accepted: false, withinReference: isWithinAlphaReference(observation) });
