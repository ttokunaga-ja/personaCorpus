/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-006793",
  sourceId: "p01-src-006793",
  artifactId: "p01-full-006793",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 187,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
