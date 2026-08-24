/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-009390",
  sourceId: "p01-src-009390",
  artifactId: "p01-full-009390",
  release: "BETA-R4",
  status: "unreviewed-inbox",
  observedP95Ms: 184,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
