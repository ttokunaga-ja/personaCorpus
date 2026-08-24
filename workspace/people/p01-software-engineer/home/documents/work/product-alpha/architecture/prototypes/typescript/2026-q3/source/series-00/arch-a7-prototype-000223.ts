// Synthetic Product Alpha R7 architecture contract: p01-src-000223
const observation = { recordId: "ARCH-A7-000223", service: "svc-release-coordinator", decisionId: "ADR-025", observedOn: "2026-07-29", p95Ms: 190, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
