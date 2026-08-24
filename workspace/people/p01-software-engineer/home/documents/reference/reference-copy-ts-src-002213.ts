/** Reference Standard P01-SRC-002213. Provenance: Aster Loop synthetic Product Engineering. */
export const reference = Object.freeze({ id: 'REF-26Q3-002213', sourceId: 'p01-src-002213', subject: 'Product Alpha R7', anchor: 'ADR-042 asynchronous release gate', reviewedAt: '2026-07-20', status: 'reviewed' as const, alphaP95Ms: 184, betaDesignBudgetMs: 225 });
export function isReviewedReference(value: typeof reference): boolean { return value.status === 'reviewed' && value.sourceId === 'p01-src-002213'; }
