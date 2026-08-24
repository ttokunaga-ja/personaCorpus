/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-004215",
  sourceId: "p01-src-004215",
  artifactId: "p01-full-004215",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 184,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
