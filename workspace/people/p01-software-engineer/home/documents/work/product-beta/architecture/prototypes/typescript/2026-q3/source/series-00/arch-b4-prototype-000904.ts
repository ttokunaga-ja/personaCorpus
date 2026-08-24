export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000904: ArchitectureReview = {
  reviewId: "ADR-B4-109",
  release: "BETA-R4",
  subject: "idempotency window",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000904.budgetMs;
}
