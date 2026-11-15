/** Synthetic attachment referenced by MSG-2026Q3-007559; no network or credentials. */
export type MailEvidence = {
  messageId: string;
  project: "BETA-R4";
  subject: string;
  designBudgetMs: number;
};

export const reviewedMailEvidence = (): MailEvidence => ({
  messageId: "MSG-2026Q3-007559",
  project: "BETA-R4",
  subject: "staging rollout observation",
  designBudgetMs: 225,
});
