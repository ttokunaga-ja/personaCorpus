/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-005817",
  sourceId: "p01-src-005817",
  artifactId: "p01-full-005817",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 186,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
