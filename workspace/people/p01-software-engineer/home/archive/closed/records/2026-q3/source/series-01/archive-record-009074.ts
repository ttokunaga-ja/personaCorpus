export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-009074";
  sourceId: "p01-src-009074";
  project: "MIG-26Q3-W01";
  closedOn: "2026-09-16";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-009074", sourceId: "p01-src-009074",
  project: "MIG-26Q3-W01", closedOn: "2026-09-16", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
