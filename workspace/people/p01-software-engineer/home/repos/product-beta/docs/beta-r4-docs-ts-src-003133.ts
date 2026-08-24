export type BetaEvidence = { recordId: string; service: string; total: number; passed: number; failed: number };

export const evidence3133: BetaEvidence = { recordId: 'BETA-DOC-003133', service: 'svc-release-coordinator', total: 1240, passed: 1231, failed: 9 };

export function isReconciled(value: BetaEvidence): boolean {
  return value.total === value.passed + value.failed && value.passed / value.total >= 0.99;
}
