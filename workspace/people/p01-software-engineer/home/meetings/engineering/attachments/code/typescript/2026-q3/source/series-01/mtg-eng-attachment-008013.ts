export const meeting = { id: "MTG-ENG-20260914-04", reference: "MIG-26Q3-W04", product: "Migration program", ownerRole: "Migration Lead", actionId: "ACT-ENG-008013", measure: "45m" } as const;
export function agendaLine(): string { return `${meeting.id}: ${meeting.reference} (${meeting.measure})`; }
