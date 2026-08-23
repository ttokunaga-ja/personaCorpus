type Run = Readonly<{ name: string; precision: number; recall: number; f1: number }>;

const runs: readonly Run[] = [
  { name: "Alpha M-14", precision: 0.881, recall: 0.865, f1: 0.873 },
  { name: "Beta B-07", precision: 0.852, recall: 0.841, f1: 0.846 },
];
const labels: Readonly<Record<string, number>> = {
  account_access: 492,
  delivery_status: 476,
  plan_change: 481,
  refund_request: 462,
  technical_help: 489,
};

function f1(precision: number, recall: number): number {
  return Number((2 * precision * recall / (precision + recall)).toFixed(3));
}

function report(): string[] {
  const total = Object.values(labels).reduce((sum, count) => sum + count, 0);
  const checks = runs.map((run) => `${run.name}: ${f1(run.precision, run.recall) === run.f1 ? "verified" : "review"}`);
  return ["Helix Forge Lab export report", "R184 · 2026-07-13", `HF-Eval-S1/S2 v3.2: ${total} examples (S1 1600, S2 800)`, ...checks];
}

console.log(report().join("\n"));
