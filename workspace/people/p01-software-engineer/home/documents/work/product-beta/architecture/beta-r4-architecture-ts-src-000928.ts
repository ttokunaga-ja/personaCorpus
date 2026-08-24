export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000928: ArchitectureReview = {
  reviewId: "ADR-B4-133",
  release: "BETA-R4",
  subject: "event envelope compatibility",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000928.budgetMs;
}
