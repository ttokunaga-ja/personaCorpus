/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-002864",
  sourceId: "p01-src-002864",
  artifactId: "p01-full-002864",
  release: "BETA-R4",
  status: "unreviewed-inbox",
  observedP95Ms: 188,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
