export interface BetaRepositoryEvidence { recordId: string; service: string; total: number; passed: number; failed: number; }
export const evidence6714: BetaRepositoryEvidence = { recordId: 'BETA-REPO-006714', service: 'svc-edge-api', total: 1240, passed: 1231, failed: 9 };
export const isReleaseReady = (value: BetaRepositoryEvidence): boolean => value.total === value.passed + value.failed && value.passed / value.total >= 0.99;
