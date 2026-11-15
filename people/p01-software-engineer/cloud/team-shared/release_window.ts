/** Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042 */
export type GateEvidence = Readonly<{ checkpointMatched: boolean; rollbackDrilled: boolean; p95Ms: number }>;
export type Decision = "open" | "hold" | "stop";

export function releaseWindow(date: string, evidence: GateEvidence): Decision {
  if (date !== "2026-07-13") return "hold";
  if (!evidence.checkpointMatched || !evidence.rollbackDrilled) return "hold";
  return evidence.p95Ms <= 200 ? "open" : "stop";
}

function assert(condition: boolean, message: string): void { if (!condition) throw new Error(message); }
export function runValidationTests(): void {
  assert(releaseWindow("2026-07-13", { checkpointMatched: true, rollbackDrilled: true, p95Ms: 184 }) === "open", "baseline opens");
  assert(releaseWindow("2026-07-13", { checkpointMatched: false, rollbackDrilled: true, p95Ms: 184 }) === "hold", "missing evidence holds");
  assert(releaseWindow("2026-07-13", { checkpointMatched: true, rollbackDrilled: true, p95Ms: 201 }) === "stop", "threshold stops");
}
runValidationTests();
