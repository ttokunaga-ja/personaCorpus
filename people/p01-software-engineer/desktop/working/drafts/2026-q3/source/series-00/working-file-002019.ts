// Ephemeral desktop working copy for p01-src-002019; synthetic only.
type WorkingObservation = { issueId: string; service: string; latencyMs: number; state: "draft" | "working" | "review-pending" | "revised" };

const observation: WorkingObservation = { issueId: "ALW-3019", service: "svc-event-bus", latencyMs: 185, state: "revised" };

export const isWithinAlphaReference = (item: WorkingObservation): boolean => item.latencyMs <= 200;
export const sourceId = "p01-src-002019";
console.log({ sourceId, accepted: false, withinReference: isWithinAlphaReference(observation) });
