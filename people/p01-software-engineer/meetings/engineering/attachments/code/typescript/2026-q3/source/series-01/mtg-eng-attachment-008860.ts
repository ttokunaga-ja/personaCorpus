export const meeting = { id: "MTG-ENG-20260817-50", reference: "VND-26Q3-14", product: "Platform portfolio", ownerRole: "Platform Engineer", actionId: "ACT-ENG-008860", measure: "438 s" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
