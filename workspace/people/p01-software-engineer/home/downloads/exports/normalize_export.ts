// Aster Loop / Product Alpha R7 / 2026-07-13 / p95 184 ms / ADR-042
export interface BuildMetric { timestamp:string; build_id:string; stage:string; duration_ms:number; status:"passed"|"failed"; p95_ms:number; anchor:string; }
export interface Review { review_id:string; stage:string; role_label:string; timestamp:string; disposition:string; anchor:string; }
export function parseMetric(row:Record<string,string>):BuildMetric { const duration_ms=Number(row.duration_ms),p95_ms=Number(row.p95_ms); if(!Number.isFinite(duration_ms)||!Number.isFinite(p95_ms)) throw new Error("numeric fields required"); if(!row.build_id||!row.anchor.includes("ADR-042")) throw new Error("required export fields missing"); return {...row,duration_ms,p95_ms,status:row.status as "passed"|"failed"}; }
export function normalizeReview(row:Review):Review { return {...row,role_label:row.role_label.trim(),disposition:row.disposition.toLowerCase()}; }
export const example=parseMetric({timestamp:"2026-07-13T08:30:00Z",build_id:"r7-081",stage:"compile",duration_ms:"42000",status:"passed",p95_ms:"184",anchor:"ADR-042"});
