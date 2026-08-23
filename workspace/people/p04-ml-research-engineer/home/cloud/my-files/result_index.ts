type Metrics = Readonly<{ precision: number; recall: number; f1: number }>;
type Result = Readonly<{ model: string; run: string; metrics: Metrics }>;

const results: Result[] = [
  { model: "Alpha M-14", run: "R184", metrics: { precision: 0.881, recall: 0.865, f1: 0.873 } },
  { model: "Beta B-07", run: "comparison", metrics: { precision: 0.852, recall: 0.841, f1: 0.846 } },
];

function winner(items: readonly Result[]): Result {
  if (items.length === 0) throw new Error("result index is empty");
  return items.reduce((best, item) => item.metrics.f1 > best.metrics.f1 ? item : best);
}

function delta(left: Metrics, right: Metrics): Metrics {
  return {
    precision: +(left.precision - right.precision).toFixed(3),
    recall: +(left.recall - right.recall).toFixed(3),
    f1: +(left.f1 - right.f1).toFixed(3),
  };
}

const champion = winner(results);
const improvement = delta(results[0].metrics, results[1].metrics);
console.log(`champion=${champion.model} ${champion.run}; F1=${champion.metrics.f1.toFixed(3)}`);
console.log(`Alpha minus Beta: precision=${improvement.precision}, recall=${improvement.recall}, f1=${improvement.f1}`);
