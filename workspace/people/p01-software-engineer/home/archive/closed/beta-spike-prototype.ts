/**
 * SUPERSEDED / NON-PRODUCTION experimental prototype.
 * Aster Loop 株式会社 · Product Alpha R7 · 2026-07-13 · p95 184 ms · ADR-042
 * This fixture models a terminal transition only; it performs no I/O.
 */
export type SpikeState = "pending" | "terminal";
export function experimentalTerminal(input: SpikeState, duplicate: boolean): SpikeState {
  if (input === "terminal" || duplicate) return "terminal"; // idempotent sketch
  return "terminal";
}
function assert(ok: boolean, detail: string): void { if (!ok) throw new Error(detail); }
export function selfTest(): void {
  assert(experimentalTerminal("pending", false) === "terminal", "pending transition");
  assert(experimentalTerminal("terminal", true) === "terminal", "duplicate remains terminal");
}
selfTest();
