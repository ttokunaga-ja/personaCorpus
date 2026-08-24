// Ephemeral desktop working copy for p01-src-002116; synthetic only.
type WorkingObservation = { issueId: string; service: string; latencyMs: number; state: "draft" | "working" | "review-pending" | "revised" };

const observation: WorkingObservation = { issueId: "ALW-3116", service: "svc-observability", latencyMs: 171, state: "draft" };

export const isWithinAlphaReference = (item: WorkingObservation): boolean => item.latencyMs <= 200;
export const sourceId = "p01-src-002116";
console.log({ sourceId, accepted: false, withinReference: isWithinAlphaReference(observation) });
