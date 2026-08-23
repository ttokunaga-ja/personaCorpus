/** Render a self-contained synthetic evidence matrix for literature review. */
declare const process: { argv: string[] } | undefined;

type EvidenceRow = { source: string; question: string; status: "supports" | "cautions" | "needs-review" };

const rows: EvidenceRow[] = [
  { source: "R184 Evaluation Ledger [synthetic/internal]", question: "Is macro F1 the ranking metric?", status: "supports" },
  { source: "HF-Eval-S1/S2 Design Card [synthetic/internal]", question: "Is partition shift documented?", status: "supports" },
  { source: "Confidence Review Note [synthetic/internal]", question: "Are threshold changes justified?", status: "needs-review" },
];

export function renderEvidenceMatrix(input: EvidenceRow[] = rows): string {
  const header = "| Source | Review question | Status |\n| --- | --- | --- |";
  const body = input.map(({ source, question, status }) => `| ${source} | ${question} | ${status} |`).join("\n");
  return `${header}\n${body}`;
}

if (typeof process !== "undefined" && process.argv[1]?.endsWith("evidence_matrix_renderer.ts")) {
  console.log(renderEvidenceMatrix());
}
