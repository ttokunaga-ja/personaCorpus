// Ephemeral desktop working copy for p01-src-003069; synthetic only.
type WorkingObservation = { issueId: string; service: string; latencyMs: number; state: "draft" | "working" | "review-pending" | "revised" };

const observation: WorkingObservation = { issueId: "ALW-4069", service: "svc-event-bus", latencyMs: 199, state: "working" };

export const isWithinAlphaReference = (item: WorkingObservation): boolean => item.latencyMs <= 200;
export const sourceId = "p01-src-003069";
console.log({ sourceId, accepted: false, withinReference: isWithinAlphaReference(observation) });
