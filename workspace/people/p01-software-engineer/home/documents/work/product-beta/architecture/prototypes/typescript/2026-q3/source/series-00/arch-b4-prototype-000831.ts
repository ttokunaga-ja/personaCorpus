export type ArchitectureReview = Readonly<{
  reviewId: string;
  release: "BETA-R4";
  subject: string;
  budgetMs: number;
}>;

export const review000831: ArchitectureReview = {
  reviewId: "ADR-B4-036",
  release: "BETA-R4",
  subject: "capacity review",
  budgetMs: 225,
};

export function isWithinBudget(elapsedMs: number): boolean {
  return elapsedMs <= review000831.budgetMs;
}
