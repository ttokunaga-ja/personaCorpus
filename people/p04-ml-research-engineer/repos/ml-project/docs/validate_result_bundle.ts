type Bundle = Readonly<{
  dataset: string;
  partitions: Readonly<Record<string, number>>;
  labels: Readonly<Record<string, number>>;
}>;

const bundle: Bundle = {
  dataset: "HF-Eval-S1/S2 v3.2",
  partitions: { "HF-Eval-S1": 1600, "HF-Eval-S2": 800 },
  labels: {
    account_access: 492, delivery_status: 476, plan_change: 481,
    refund_request: 462, technical_help: 489,
  },
};

function sum(values: Readonly<Record<string, number>>): number {
  return Object.values(values).reduce((total, value) => total + value, 0);
}

const recordTotal = sum(bundle.labels);
if (sum(bundle.partitions) !== 2400 || recordTotal !== 2400 || Object.keys(bundle.labels).length !== 5) {
  throw new Error("synthetic result bundle violates its fixed count contract");
}
console.log(`${bundle.dataset} validated: ${recordTotal} synthetic records across five labels`);
