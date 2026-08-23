type Metric = "macro_f1" | "precision" | "recall";
type Series = { model: "Alpha" | "Beta"; values: Record<Metric, number> };

export const evaluationSeries: Series[] = [
  { model: "Alpha", values: { macro_f1: 0.873, precision: 0.881, recall: 0.865 } },
  { model: "Beta", values: { macro_f1: 0.846, precision: 0.852, recall: 0.841 } },
];

export function yDomain(series = evaluationSeries): [number, number] {
  const values = series.flatMap((row) => Object.values(row.values));
  return [Math.floor(Math.min(...values) * 100) / 100, 0.9];
}

console.log(JSON.stringify({ population: 2400, split: { S1: 1600, S2: 800 }, yDomain: yDomain() }));
