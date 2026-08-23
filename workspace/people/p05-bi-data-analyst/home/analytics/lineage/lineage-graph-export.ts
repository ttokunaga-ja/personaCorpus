type Node = { id: string; type: "source" | "staging" | "metric_mart" };
type Edge = { from: string; to: string; operation: string };

export const lineageNodes: Node[] = [
  { id: "src_sales_order_lines", type: "source" },
  { id: "stg_order_lines", type: "staging" },
  { id: "mart_daily_gmv", type: "metric_mart" },
];
export const lineageEdges: Edge[] = [
  { from: "src_sales_order_lines", to: "stg_order_lines", operation: "ingest_validate" },
  { from: "stg_order_lines", to: "mart_daily_gmv", operation: "aggregate_daily_gmv" },
];

export function toDot(): string {
  return ["digraph MTR09 {", ...lineageEdges.map(e => `  ${e.from} -> ${e.to} [label=\"${e.operation}\"];`), "}"].join("\n");
}
