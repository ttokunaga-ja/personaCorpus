export const meeting = { id: "MTG-ENG-20260817-47", reference: "VND-26Q3-14", product: "Platform portfolio", ownerRole: "Platform Engineer", actionId: "ACT-ENG-011260", measure: "438 s" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
