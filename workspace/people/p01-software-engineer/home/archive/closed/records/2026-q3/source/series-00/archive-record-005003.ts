export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-005003";
  sourceId: "p01-src-005003";
  project: "VND-26Q3";
  closedOn: "2026-09-10";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-005003", sourceId: "p01-src-005003",
  project: "VND-26Q3", closedOn: "2026-09-10", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
