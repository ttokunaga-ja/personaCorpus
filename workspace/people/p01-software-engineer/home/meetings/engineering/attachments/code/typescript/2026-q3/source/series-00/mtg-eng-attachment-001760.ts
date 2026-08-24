export const meeting = { id: "MTG-ENG-20260803-70", reference: "ADR-057", product: "Product Alpha R7", ownerRole: "Observability Lead", actionId: "ACT-ENG-001760", measure: "200 ms" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
