// Ephemeral desktop working copy for p01-src-002096; synthetic only.
type WorkingObservation = { issueId: string; service: string; latencyMs: number; state: "draft" | "working" | "review-pending" | "revised" };

const observation: WorkingObservation = { issueId: "ALW-3096", service: "svc-decision-store", latencyMs: 188, state: "draft" };

export const isWithinAlphaReference = (item: WorkingObservation): boolean => item.latencyMs <= 200;
export const sourceId = "p01-src-002096";
console.log({ sourceId, accepted: false, withinReference: isWithinAlphaReference(observation) });
