export type ClosedRecord = Readonly<{
  recordId: "ARC-26Q3-007969";
  sourceId: "p01-src-007969";
  project: "BETA-R4";
  closedOn: "2026-09-22";
  state: "closed";
}>;

export const closedRecord: ClosedRecord = {
  recordId: "ARC-26Q3-007969", sourceId: "p01-src-007969",
  project: "BETA-R4", closedOn: "2026-09-22", state: "closed",
};

export const isImmutable = (item: ClosedRecord): boolean => item.state === "closed";
