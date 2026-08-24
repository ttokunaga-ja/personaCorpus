export const meeting = { id: "MTG-ENG-20260904-72", reference: "API-B4-120", product: "Product Beta R4", ownerRole: "API Steward", actionId: "ACT-ENG-006835", measure: "225 ms" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
