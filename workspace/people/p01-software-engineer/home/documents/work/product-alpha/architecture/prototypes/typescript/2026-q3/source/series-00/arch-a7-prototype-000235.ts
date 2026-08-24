// Synthetic Product Alpha R7 architecture contract: p01-src-000235
const observation = { recordId: "ARCH-A7-000235", service: "svc-release-coordinator", decisionId: "ADR-037", observedOn: "2026-08-10", p95Ms: 183, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
