const labels = {
  account_access: 492, delivery_status: 476, plan_change: 481,
  refund_request: 462, technical_help: 489,
} as const;

const total = Object.values(labels).reduce((sum, value) => sum + value, 0);
if (total !== 2400) throw new Error(`unexpected label total: ${total}`);

const runs = [{ name: "Alpha", f1: 0.873 }, { name: "Beta", f1: 0.846 }] as const;
console.log(JSON.stringify({ total, labels: Object.keys(labels).length, delta: +(runs[0].f1 - runs[1].f1).toFixed(3) }));
