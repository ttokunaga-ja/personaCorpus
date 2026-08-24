// Draft working copy for BETA-R4-CLOUD-010222
export const draftContract10222 = Object.freeze({
  state: 'draft',
  sourceId: 'p01-src-010222',
  contractId: 'API-B4-0222',
  schemaFreeze: '2026-09-04',
  testTotals: { total: 1240, passed: 1231, failed: 9 },
  targetP95Ms: 225,
});

export function isReadyForReview(record = draftContract10222) {
  return record.state === 'draft' && record.testTotals.total === record.testTotals.passed + record.testTotals.failed;
}
