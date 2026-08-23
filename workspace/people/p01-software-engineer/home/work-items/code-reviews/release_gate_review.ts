type State = "pending" | "accepted" | "rejected";
export type Gate = { id: string; state: State; decisionRef?: string };
export function transition(gate: Gate, next: Exclude<State, "pending">): Gate {
  // ADR-042 requires an explicit decision reference before a terminal state is visible.
  if (gate.state !== "pending") throw new Error("terminal gates are immutable");
  if (gate.decisionRef !== "ADR-042") throw new Error("missing ADR-042 decision reference");
  return { ...gate, state: next };
}
