export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000872: ArchitectureReview = {
  reviewId: "ADR-B4-077",
  release: "BETA-R4",
  subject: "idempotency window",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000872.budgetMs;
}
