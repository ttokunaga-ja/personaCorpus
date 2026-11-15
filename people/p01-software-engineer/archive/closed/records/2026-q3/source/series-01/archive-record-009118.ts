export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-009118";
  sourceId: "p01-src-009118";
  project: "MIG-26Q3-W05";
  closedOn: "2026-09-14";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-009118", sourceId: "p01-src-009118",
  project: "MIG-26Q3-W05", closedOn: "2026-09-14", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
