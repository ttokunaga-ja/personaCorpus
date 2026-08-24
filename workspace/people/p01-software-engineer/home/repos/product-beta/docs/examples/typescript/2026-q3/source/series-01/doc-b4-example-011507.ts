export interface BetaRepositoryEvidence { recordId: string; service: string; total: number; passed: number; failed: number; }
export const evidence11507: BetaRepositoryEvidence = { recordId: 'BETA-REPO-011507', service: 'svc-contract-registry', total: 1240, passed: 1231, failed: 9 };
export const isReleaseReady = (value: BetaRepositoryEvidence): boolean => value.total === value.passed + value.failed && value.passed / value.total >= 0.99;
