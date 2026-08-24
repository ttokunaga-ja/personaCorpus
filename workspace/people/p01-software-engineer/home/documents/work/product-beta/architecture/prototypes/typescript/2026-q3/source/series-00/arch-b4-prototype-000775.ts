export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000775: ArchitectureReview = {
  reviewId: "ADR-B4-179",
  release: "BETA-R4",
  subject: "replay safety",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000775.budgetMs;
}
