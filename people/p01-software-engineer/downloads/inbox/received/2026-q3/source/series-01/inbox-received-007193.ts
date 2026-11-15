/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-007193",
  sourceId: "p01-src-007193",
  artifactId: "p01-full-007193",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 187,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
