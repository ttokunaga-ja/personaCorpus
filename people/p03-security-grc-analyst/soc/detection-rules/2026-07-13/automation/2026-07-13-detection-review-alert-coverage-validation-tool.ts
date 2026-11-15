/** Synthetic AC-27 evidence validator for p03-full-005361; no live-system access. */
export type ReviewMetrics = { reviewed: number; completed: number; required: number; reconciled: number };
export const isEvidenceComplete = (metrics: ReviewMetrics): boolean =>
  metrics.reviewed === 48 && metrics.completed === 48 && metrics.required === 24 && metrics.reconciled === 24;

export const reviewContext = {"decision": "SOC duty review confirms the minimized Sentinel Export boundary; CASE-260713-184 retains RISK-031 at Medium residual risk.", "evidence_reference": "evidence set 184, reconciliation lane 005361", "review_date_or_period": "2026-07-13", "review_topic": "alert coverage", "synthetic_boundary": "defensive governance record only; no live telemetry, credentials, personal data, or routable targets"} as const;
export const contentSpineTraceability = {"date.2026-07-13": "Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.", "org.cobalt-harbor": "Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.", "team.soc": "Security Operations owns bounded SIEM exports, detection review evidence, and incident evidence-chain handoff."} as const;
