type SyntheticClaim = {
  claimId: string;
  recordId: string;
  sourceRoleId: "Source Alpha-01" | "Source Beta-01";
};

const exampleClaims: SyntheticClaim[] = [
  { claimId: "CLM-A-01", recordId: "CL-A17-2026-04", sourceRoleId: "Source Alpha-01" },
  { claimId: "CLM-B-01", recordId: "CL-B09-2026-11", sourceRoleId: "Source Beta-01" },
];

export function hasUniqueRecordLinks(claims: SyntheticClaim[]): boolean {
  return new Set(claims.map((claim) => claim.recordId)).size === claims.length;
}

if (!hasUniqueRecordLinks(exampleClaims)) {
  throw new Error("Synthetic claim links must use unique record IDs.");
}
