// Aster Loop / Product Alpha R7 / 2026-07-13 / p95 184 ms / ADR-042
export interface Span { stage: string; ms: number; }
export function percentile(values: number[], p: number): number { const sorted=[...values].sort((a,b)=>a-b); return sorted[Math.ceil(p*sorted.length)-1]; }
export function probeFixture(): { p95:number; spans:Span[] } { const spans=[{stage:"edge",ms:28},{stage:"validation",ms:32},{stage:"admission",ms:46},{stage:"publish",ms:24},{stage:"observability",ms:18},{stage:"reserve",ms:36}]; return {p95:percentile([160,170,176,180,184],.95),spans}; }
