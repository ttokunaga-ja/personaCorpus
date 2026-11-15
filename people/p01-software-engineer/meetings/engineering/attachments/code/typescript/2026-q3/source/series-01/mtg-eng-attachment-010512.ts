export const meeting = { id: "MTG-ENG-20260720-11", reference: "ADR-042", product: "Product Alpha R7", ownerRole: "Release Coordinator", actionId: "ACT-ENG-010512", measure: "184 ms" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
