export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-010525";
  sourceId: "p01-src-010525";
  project: "BETA-R4";
  closedOn: "2026-09-27";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-010525", sourceId: "p01-src-010525",
  project: "BETA-R4", closedOn: "2026-09-27", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
