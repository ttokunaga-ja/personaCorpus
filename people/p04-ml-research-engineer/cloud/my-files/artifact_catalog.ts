type Artifact = {
  key: string;
  purpose: string;
  retentionDays: number;
  sha6: string;
};

const catalog: Artifact[] = [
  { key: "eval/R184/metrics.json", purpose: "Alpha M-14 aggregate metrics", retentionDays: 365, sha6: "9ac27e" },
  { key: "eval/R184/predictions.jsonl", purpose: "held-out prediction trace", retentionDays: 90, sha6: "bc8154" },
  { key: "eval/R184/error-slices.csv", purpose: "label-level review slice", retentionDays: 180, sha6: "6d3e01" },
];

function catalogByRetention(days: number): string[] {
  return catalog.filter((artifact) => artifact.retentionDays >= days).map((artifact) => artifact.key);
}

function isSha6(value: string): boolean {
  return /^[0-9a-f]{6}$/i.test(value);
}

for (const artifact of catalog) {
  if (!isSha6(artifact.sha6)) throw new Error(`invalid catalog checksum for ${artifact.key}`);
}

console.log(`Helix Forge Lab catalog: ${catalog.length} artifacts`);
console.log(`long retention: ${catalogByRetention(180).join(", ")}`);
