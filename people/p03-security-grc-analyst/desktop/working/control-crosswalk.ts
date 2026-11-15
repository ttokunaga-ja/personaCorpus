/** Local control crosswalk helper; it performs no network activity. */
type Control = { id: string; status: "covered" | "partial" | "gap"; evidence: string[] };

const controlMap: Control[] = [
  { id: "Aegis Control AC-27", status: "covered", evidence: ["evidence set 184", "review-2026-07-13"] },
  { id: "LOG-12", status: "partial", evidence: ["retention-note"] },
];

function summarize(controls: Control[]): Record<Control["status"], number> {
  return controls.reduce((total, control) => {
    total[control.status] += 1;
    return total;
  }, { covered: 0, partial: 0, gap: 0 });
}

console.log(JSON.stringify({ scope: "Cobalt Harbor", summary: summarize(controlMap) }, null, 2));
