export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000883: ArchitectureReview = {
  reviewId: "ADR-B4-088",
  release: "BETA-R4",
  subject: "deployment lane isolation",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000883.budgetMs;
}
