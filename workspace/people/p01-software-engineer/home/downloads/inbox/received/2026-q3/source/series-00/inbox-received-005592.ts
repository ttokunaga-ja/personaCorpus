/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-005592",
  sourceId: "p01-src-005592",
  artifactId: "p01-full-005592",
  release: "BETA-R4",
  status: "unreviewed-inbox",
  observedP95Ms: 186,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
