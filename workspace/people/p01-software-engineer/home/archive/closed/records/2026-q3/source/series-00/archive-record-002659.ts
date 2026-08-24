export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-002659";
  sourceId: "p01-src-002659";
  project: "BETA-R4";
  closedOn: "2026-09-18";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-002659", sourceId: "p01-src-002659",
  project: "BETA-R4", closedOn: "2026-09-18", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
