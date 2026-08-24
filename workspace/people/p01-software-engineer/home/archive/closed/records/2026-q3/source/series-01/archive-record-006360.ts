export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-006360";
  sourceId: "p01-src-006360";
  project: "ALPHA-R7";
  closedOn: "2026-07-18";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-006360", sourceId: "p01-src-006360",
  project: "ALPHA-R7", closedOn: "2026-07-18", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
