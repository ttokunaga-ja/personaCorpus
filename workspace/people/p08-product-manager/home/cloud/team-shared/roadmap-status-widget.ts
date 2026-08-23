/** Offline render-model helpers for the Product Alpha Q3 roadmap. */
export type RoadmapStatus = "complete" | "on_track" | "at_risk" | "planned";

export interface RoadmapMilestone {
  id: "M-24" | "M-25" | "M-26" | "M-27";
  name: string;
  date: string;
  status: RoadmapStatus;
  ownerRole: string;
  dependsOn: readonly string[];
}

export interface StatusWidgetModel {
  title: string;
  asOf: string;
  adoption: { baselinePercent: number; targetPercent: number; deltaPoints: number };
  syntheticWorkspaces: number;
  rows: ReadonlyArray<{
    id: string;
    label: string;
    date: string;
    status: RoadmapStatus;
    statusLabel: string;
    ownerRole: string;
    dependencyLabel: string;
  }>;
}

const STATUS_LABEL: Readonly<Record<RoadmapStatus, string>> = {
  complete: "Complete",
  on_track: "On track",
  at_risk: "At risk",
  planned: "Planned",
};

export function buildStatusWidgetModel(
  milestones: readonly RoadmapMilestone[],
  asOf = "2026-07-13",
): StatusWidgetModel {
  const rows = [...milestones]
    .sort((left, right) => left.date.localeCompare(right.date))
    .map((milestone) => ({
      id: milestone.id,
      label: milestone.name,
      date: milestone.date,
      status: milestone.status,
      statusLabel: STATUS_LABEL[milestone.status],
      ownerRole: milestone.ownerRole,
      dependencyLabel:
        milestone.dependsOn.length === 0
          ? "No predecessor"
          : `Depends on ${milestone.dependsOn.join(", ")}`,
    }));

  return {
    title: "Juniper Works — Product Alpha Q3 roadmap",
    asOf,
    adoption: { baselinePercent: 18.4, targetPercent: 26.0, deltaPoints: 7.6 },
    syntheticWorkspaces: 1240,
    rows,
  };
}
