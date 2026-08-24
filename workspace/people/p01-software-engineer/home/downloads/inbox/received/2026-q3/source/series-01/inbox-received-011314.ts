/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-011314",
  sourceId: "p01-src-011314",
  artifactId: "p01-full-011314",
  release: "BETA-R4",
  status: "unreviewed-inbox",
  observedP95Ms: 188,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
