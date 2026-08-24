/** Synthetic attachment referenced by MSG-2026Q3-008468; no network or credentials. */
export type MailEvidence = {
  messageId: string;
  project: "BETA-R4";
  subject: string;
  designBudgetMs: number;
};

export const reviewedMailEvidence = (): MailEvidence => ({
  messageId: "MSG-2026Q3-008468",
  project: "BETA-R4",
  subject: "decision record follow-up",
  designBudgetMs: 225,
});
