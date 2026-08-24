export const meeting = { id: "MTG-ENG-20260824-02", reference: "ADR-B4-031", product: "Product Beta R4", ownerRole: "Decision Owner", actionId: "ACT-ENG-001781", measure: "225 ms" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
