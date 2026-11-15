/* Synthetic Cedar Works ERP export normalizer; no dependencies. */
declare const require: any;
declare const process: { argv: string[]; exitCode?: number };
const fs = require("fs");

type Row = Record<string, string>;
type NormalizedRow = {
  period: string; accountCode: string; accountName: string; amountJpy: number;
  debitJpy: number; creditJpy: number; currency: string; source: string;
};

function parseDelimited(text: string, delimiter: string): Row[] {
  const lines = text.trim().split(/\r?\n/);
  const parseLine = (line: string): string[] => {
    const out: string[] = []; let value = ""; let quoted = false;
    for (let i = 0; i < line.length; i += 1) {
      const ch = line[i];
      if (ch === '"') { if (quoted && line[i + 1] === '"') { value += ch; i += 1; } else quoted = !quoted; }
      else if (ch === delimiter && !quoted) { out.push(value); value = ""; }
      else value += ch;
    }
    out.push(value); return out;
  };
  const headers = parseLine(lines[0]);
  return lines.slice(1).filter(Boolean).map((line) => Object.fromEntries(parseLine(line).map((v, i) => [headers[i], v])));
}

function num(value: string | undefined): number { return Number(value ?? 0); }
export function normalizeExport(text: string, filename: string): NormalizedRow[] {
  const delimiter = filename.toLowerCase().endsWith(".tsv") ? "\t" : ",";
  return parseDelimited(text, delimiter).map((row) => ({
    period: row.posting_date ?? row.period ?? "",
    accountCode: row.account_code ?? row.line_item ?? "",
    accountName: row.account_name ?? row.line_item ?? "",
    amountJpy: num(row.actual_jpy ?? row.net_jpy),
    debitJpy: num(row.debit_jpy), creditJpy: num(row.credit_jpy),
    currency: row.currency ?? "JPY", source: filename,
  }));
}
if (process.argv[2]) {
  const input = process.argv[2];
  console.log(JSON.stringify(normalizeExport(fs.readFileSync(input, "utf8"), input), null, 2));
}
