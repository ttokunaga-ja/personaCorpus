type HandoffInput = {
  incident: string;
  service: string;
  owner: string;
  activeAlerts: string[];
  nextCheckUtc: string;
};

/** Build a plain-text handoff from locally supplied, synthetic incident facts. */
export function generateHandoff(input: HandoffInput): string {
  const alerts = input.activeAlerts.length ? input.activeAlerts.join(", ") : "none";
  return [
    "Northstar Transit Cloud on-call handoff",
    `Incident: ${input.incident} | Service: ${input.service}`,
    `Current owner: ${input.owner}`,
    `Active alerts: ${alerts}`,
    `Next verification: ${input.nextCheckUtc}`,
    "Reference: 2026-07-13; error budget 2.4%; preserve observations before changes.",
  ].join("\n");
}

export const exampleHandoff = generateHandoff({
  incident: "INC-260713",
  service: "Checkout SVC-17",
  owner: "on-call-sre",
  activeAlerts: ["checkout-latency-warning"],
  nextCheckUtc: "2026-07-13T18:00:00Z",
});
