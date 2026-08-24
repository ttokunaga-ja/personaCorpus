/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-003815",
  sourceId: "p01-src-003815",
  artifactId: "p01-full-003815",
  release: "ALPHA-R7",
  status: "unreviewed-inbox",
  observedP95Ms: 184,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
