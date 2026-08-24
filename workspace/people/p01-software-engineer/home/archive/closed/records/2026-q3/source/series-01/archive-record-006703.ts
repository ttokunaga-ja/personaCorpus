export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-006703";
  sourceId: "p01-src-006703";
  project: "BETA-R4";
  closedOn: "2026-09-24";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-006703", sourceId: "p01-src-006703",
  project: "BETA-R4", closedOn: "2026-09-24", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
