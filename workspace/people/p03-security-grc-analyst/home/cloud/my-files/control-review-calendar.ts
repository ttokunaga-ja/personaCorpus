/** Local planning data for fictional control reviews; it performs no I/O. */
type Review = Readonly<{
  date: string;
  control: string;
  purposeJa: string;
  purposeEn: string;
}>;

const reviews: readonly Review[] = [
  { date: "2026-07-13", control: "Aegis Control AC-27", purposeJa: "証跡セットの完全性確認", purposeEn: "verify evidence-set completeness" },
  { date: "2026-07-20", control: "AC-18", purposeJa: "委託先回答の照合", purposeEn: "reconcile vendor responses" },
  { date: "2026-07-27", control: "AC-11", purposeJa: "保持判断のレビュー", purposeEn: "review retention decisions" },
];

export function upcoming(after: string): readonly Review[] {
  return reviews.filter((review) => review.date >= after);
}

export function renderCalendar(): string {
  return [
    "Cobalt Harbor control calendar / 防御・監査用の合成計画",
    "RISK-031 is reviewed with evidence set 184.",
    ...upcoming("2026-07-13").map((review) => `${review.date} | ${review.control} | ${review.purposeEn} / ${review.purposeJa}`),
  ].join("\n");
}
