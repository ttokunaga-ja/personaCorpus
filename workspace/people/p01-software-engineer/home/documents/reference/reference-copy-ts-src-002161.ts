/** Reference Standard P01-SRC-002161. Provenance: Aster Loop synthetic Product Engineering. */
export const reference = Object.freeze({ id: 'REF-26Q3-002161', sourceId: 'p01-src-002161', subject: 'Product Alpha R7', anchor: 'ADR-042 asynchronous release gate', reviewedAt: '2026-08-17', status: 'reviewed' as const, alphaP95Ms: 184, betaDesignBudgetMs: 225 });
export function isReviewedReference(value: typeof reference): boolean { return value.status === 'reviewed' && value.sourceId === 'p01-src-002161'; }
