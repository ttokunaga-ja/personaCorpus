/** Reviewed Aster Loop team-shared helper for p01-full-007701. */
type CollaborationRecord = Readonly<{ artifactId: string; sourceId: string; owner: string; reviewer: string; status: 'reviewed' }>;
export const record: CollaborationRecord = { artifactId: 'p01-full-007701', sourceId: 'p01-src-007701', owner: 'Decision Owner', reviewer: 'Migration Lead', status: 'reviewed' };
export const describe = (): string => `TS-1147: ${record.owner} accepted rollback drill follow-up; ADR-042 retained.`;
