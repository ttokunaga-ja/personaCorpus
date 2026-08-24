// Synthetic Product Alpha R7 API contract: API-A7-0321
export const contract = Object.freeze({
  contractId: "API-A7-0321",
  resource: "decisions",
  operation: "evaluate",
  release: "ALPHA-R7",
  targetP95Ms: 200,
  reviewedOn: "2026-07-20"
});

export function responseFor(recordId) {
  return { recordId, contractId: contract.contractId, status: "reviewed" };
}
