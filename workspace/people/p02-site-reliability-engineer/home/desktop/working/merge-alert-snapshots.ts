type AlertSnapshot = {
  fingerprint: string;
  observedAt: string;
  severity: "critical" | "warning" | "info";
  state: "firing" | "resolved";
  service: string;
};

/** Merge local alert snapshots, retaining the most recently observed state. */
export function mergeAlertSnapshots(snapshots: AlertSnapshot[]): AlertSnapshot[] {
  const latest = new Map<string, AlertSnapshot>();
  for (const snapshot of snapshots) {
    const prior = latest.get(snapshot.fingerprint);
    if (!prior || Date.parse(snapshot.observedAt) >= Date.parse(prior.observedAt)) {
      latest.set(snapshot.fingerprint, snapshot);
    }
  }
  return [...latest.values()].sort((left, right) =>
    left.service.localeCompare(right.service) || left.fingerprint.localeCompare(right.fingerprint),
  );
}

export const incidentContext = {
  organization: "Northstar Transit Cloud",
  service: "Checkout SVC-17",
  date: "2026-07-13",
  errorBudget: "error budget 2.4%",
  incident: "INC-260713",
};
