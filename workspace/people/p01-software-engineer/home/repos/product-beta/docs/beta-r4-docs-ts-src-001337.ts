export type BetaEvidence = { recordId: string; service: string; total: number; passed: number; failed: number };

export const evidence1337: BetaEvidence = { recordId: 'BETA-DOC-001337', service: 'svc-contract-registry', total: 1240, passed: 1231, failed: 9 };

export function isReconciled(value: BetaEvidence): boolean {
  return value.total === value.passed + value.failed && value.passed / value.total >= 0.99;
}
