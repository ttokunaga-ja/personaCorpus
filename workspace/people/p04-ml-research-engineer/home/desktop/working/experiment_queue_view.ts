type Experiment = Readonly<{ name: string; split: "S1" | "S2"; objective: string; priority: number }>;

const queue: Experiment[] = [
  { name: "threshold-sweep", split: "S2", objective: "calibrate abstention", priority: 1 },
  { name: "error-slices", split: "S1", objective: "inspect label confusion", priority: 2 },
  { name: "delta-review", split: "S2", objective: "compare Alpha and Beta", priority: 3 },
];

for (const item of [...queue].sort((left, right) => left.priority - right.priority)) {
  console.log(`${item.priority}. ${item.name} [${item.split}] — ${item.objective}`);
}
