/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-005038",
  sourceId: "p01-src-005038",
  artifactId: "p01-full-005038",
  release: "BETA-R4",
  status: "unreviewed-inbox",
  observedP95Ms: 187,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
