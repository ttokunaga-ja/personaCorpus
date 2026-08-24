/** Reference Standard P01-SRC-002226. Provenance: Aster Loop synthetic Product Engineering. */
export const reference = Object.freeze({ id: 'REF-26Q3-002226', sourceId: 'p01-src-002226', subject: 'Product Beta R4', anchor: 'Beta R4 contract review', reviewedAt: '2026-09-04', status: 'reviewed' as const, alphaP95Ms: 184, betaDesignBudgetMs: 225 });
export function isReviewedReference(value: typeof reference): boolean { return value.status === 'reviewed' && value.sourceId === 'p01-src-002226'; }
