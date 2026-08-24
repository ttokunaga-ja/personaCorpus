/** Synthetic attachment referenced by MSG-2026Q3-002613; no network or credentials. */
export type MailEvidence = {
  messageId: string;
  project: "BETA-R4";
  subject: string;
  designBudgetMs: number;
};

export const reviewedMailEvidence = (): MailEvidence => ({
  messageId: "MSG-2026Q3-002613",
  project: "BETA-R4",
  subject: "migration wave handoff",
  designBudgetMs: 225,
});
