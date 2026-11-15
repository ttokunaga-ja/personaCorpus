export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000895: ArchitectureReview = {
  reviewId: "ADR-B4-100",
  release: "BETA-R4",
  subject: "capacity review",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000895.budgetMs;
}
