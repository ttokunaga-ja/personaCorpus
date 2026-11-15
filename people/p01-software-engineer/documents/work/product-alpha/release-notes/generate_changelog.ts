type Change = { id: string; area: string; kind: "breaking" | "feature" | "fix"; summary: string };

export function generateChangelog(changes: readonly Change[]): string {
  const order: Change["kind"][] = ["breaking", "feature", "fix"];
  const titles: Record<Change["kind"], string> = {
    breaking: "Breaking changes", feature: "Highlights", fix: "Fixes",
  };
  const sorted = [...changes].sort((a, b) => a.id.localeCompare(b.id));
  return order.flatMap(kind => {
    const group = sorted.filter(change => change.kind === kind);
    return group.length ? [`## ${titles[kind]}`, ...group.map(c => `- **${c.id}** (${c.area}): ${c.summary}`), ""] : [];
  }).join("\n").trimEnd() + "\n";
}
