export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000864: ArchitectureReview = {
  reviewId: "ADR-B4-069",
  release: "BETA-R4",
  subject: "event envelope compatibility",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000864.budgetMs;
}
