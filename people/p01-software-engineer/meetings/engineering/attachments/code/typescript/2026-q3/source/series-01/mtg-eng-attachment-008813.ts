export const meeting = { id: "MTG-ENG-20260824-03", reference: "ADR-B4-031", product: "Product Beta R4", ownerRole: "Decision Owner", actionId: "ACT-ENG-008813", measure: "225 ms" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
