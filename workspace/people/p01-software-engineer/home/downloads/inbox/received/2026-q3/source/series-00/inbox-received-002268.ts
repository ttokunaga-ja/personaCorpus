/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-002268",
  sourceId: "p01-src-002268",
  artifactId: "p01-full-002268",
  release: "BETA-R4",
  status: "unreviewed-inbox",
  observedP95Ms: 187,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
