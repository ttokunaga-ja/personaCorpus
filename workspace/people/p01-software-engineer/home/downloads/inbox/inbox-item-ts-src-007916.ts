/** Synthetic inbox attachment; it is not an accepted engineering record. */
export const incomingMaterial = {
  event: "INBOX-007916",
  sourceId: "p01-src-007916",
  artifactId: "p01-full-007916",
  release: "BETA-R4",
  status: "unreviewed-inbox",
  observedP95Ms: 185,
  accepted: false,
} as const;

export function requiresReview() {
  return incomingMaterial.status === "unreviewed-inbox";
}
