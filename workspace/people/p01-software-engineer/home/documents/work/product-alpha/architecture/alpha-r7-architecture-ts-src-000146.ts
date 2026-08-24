// Synthetic Product Alpha R7 architecture contract: p01-src-000146
const observation = { recordId: "ARCH-A7-000146", service: "svc-decision-store", decisionId: "ADR-147", observedOn: "2026-07-21", p95Ms: 189, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
