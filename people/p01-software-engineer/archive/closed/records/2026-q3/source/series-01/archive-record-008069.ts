export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-008069";
  sourceId: "p01-src-008069";
  project: "VND-26Q3";
  closedOn: "2026-09-08";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-008069", sourceId: "p01-src-008069",
  project: "VND-26Q3", closedOn: "2026-09-08", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
