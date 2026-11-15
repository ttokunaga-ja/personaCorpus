export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000763: ArchitectureReview = {
  reviewId: "ADR-B4-167",
  release: "BETA-R4",
  subject: "failure classification",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000763.budgetMs;
}
