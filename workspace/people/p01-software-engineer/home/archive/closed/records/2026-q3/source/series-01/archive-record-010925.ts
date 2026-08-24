export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-010925";
  sourceId: "p01-src-010925";
  project: "VND-26Q3";
  closedOn: "2026-09-13";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-010925", sourceId: "p01-src-010925",
  project: "VND-26Q3", closedOn: "2026-09-13", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
