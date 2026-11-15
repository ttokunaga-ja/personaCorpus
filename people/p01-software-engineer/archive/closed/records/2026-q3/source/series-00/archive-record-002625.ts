export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-002625";
  sourceId: "p01-src-002625";
  project: "MIG-26Q3-W03";
  closedOn: "2026-09-09";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-002625", sourceId: "p01-src-002625",
  project: "MIG-26Q3-W03", closedOn: "2026-09-09", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
