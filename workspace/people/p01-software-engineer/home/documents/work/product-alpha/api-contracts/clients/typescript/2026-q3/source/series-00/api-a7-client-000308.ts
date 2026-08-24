// Synthetic Product Alpha R7 API contract: API-A7-0308
export const contract = Object.freeze({
  contractId: "API-A7-0308",
  resource: "release-gates",
  operation: "register",
  release: "ALPHA-R7",
  targetP95Ms: 200,
  reviewedOn: "2026-07-20"
});

export function responseFor(recordId) {
  return { recordId, contractId: contract.contractId, status: "reviewed" };
}
