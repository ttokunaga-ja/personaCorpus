export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000805: ArchitectureReview = {
  reviewId: "ADR-B4-010",
  release: "BETA-R4",
  subject: "schema evolution review",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000805.budgetMs;
}
