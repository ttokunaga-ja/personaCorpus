// Ephemeral desktop working copy for p01-src-003469; synthetic only.
type WorkingObservation = { issueId: string; service: string; latencyMs: number; state: "draft" | "working" | "review-pending" | "revised" };

const observation: WorkingObservation = { issueId: "ALW-4469", service: "svc-release-coordinator", latencyMs: 192, state: "working" };

export const isWithinAlphaReference = (item: WorkingObservation): boolean => item.latencyMs <= 200;
export const sourceId = "p01-src-003469";
console.log({ sourceId, accepted: false, withinReference: isWithinAlphaReference(observation) });
