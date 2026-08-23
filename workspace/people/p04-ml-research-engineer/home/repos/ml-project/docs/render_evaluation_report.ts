type ModelResult = Readonly<{
  name: string;
  macroF1: number;
  precision: number;
  recall: number;
}>;

const alpha: ModelResult = { name: "Model Alpha M-14", macroF1: 0.873, precision: 0.881, recall: 0.865 };
const beta: ModelResult = { name: "Model Beta B-07", macroF1: 0.846, precision: 0.852, recall: 0.841 };

function render(result: ModelResult): string {
  return `${result.name}: macro F1 ${result.macroF1.toFixed(3)} ` +
    `(precision ${result.precision.toFixed(3)}, recall ${result.recall.toFixed(3)})`;
}

const report = ["Helix Forge Lab synthetic evaluation", render(alpha), render(beta),
  `delta ${(alpha.macroF1 - beta.macroF1).toFixed(3)}`].join("\n");
if (!report.includes("delta 0.027")) throw new Error("unexpected comparison");
console.log(report);
