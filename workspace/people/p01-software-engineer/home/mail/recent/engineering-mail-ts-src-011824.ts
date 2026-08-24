/** Synthetic attachment referenced by MSG-2026Q3-011824; no network or credentials. */
export type MailEvidence = {
  messageId: string;
  project: "BETA-R4";
  subject: string;
  designBudgetMs: number;
};

export const reviewedMailEvidence = (): MailEvidence => ({
  messageId: "MSG-2026Q3-011824",
  project: "BETA-R4",
  subject: "Beta R4 schema freeze note",
  designBudgetMs: 225,
});
