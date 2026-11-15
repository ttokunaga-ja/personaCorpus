// Ephemeral desktop working copy for p01-src-003306; synthetic only.
type WorkingObservation = { issueId: string; service: string; latencyMs: number; state: "draft" | "working" | "review-pending" | "revised" };

const observation: WorkingObservation = { issueId: "ALW-4306", service: "svc-edge-api", latencyMs: 177, state: "review-pending" };

export const isWithinAlphaReference = (item: WorkingObservation): boolean => item.latencyMs <= 200;
export const sourceId = "p01-src-003306";
console.log({ sourceId, accepted: false, withinReference: isWithinAlphaReference(observation) });
