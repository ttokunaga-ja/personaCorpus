/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-009615",
  sourceId: "p01-src-009615",
  artifactId: "p01-full-009615",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 184,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
