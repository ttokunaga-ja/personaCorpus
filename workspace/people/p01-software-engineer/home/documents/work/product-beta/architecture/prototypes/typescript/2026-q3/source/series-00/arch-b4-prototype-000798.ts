export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000798: ArchitectureReview = {
  reviewId: "ADR-B4-003",
  release: "BETA-R4",
  subject: "policy evaluation ordering",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000798.budgetMs;
}
