/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-002259",
  sourceId: "p01-src-002259",
  artifactId: "p01-full-002259",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 188,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
