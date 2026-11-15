/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-002311",
  sourceId: "p01-src-002311",
  artifactId: "p01-full-002311",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 185,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
