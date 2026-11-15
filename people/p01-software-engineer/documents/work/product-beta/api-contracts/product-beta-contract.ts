export type Brand<T, Name extends string> = T & { readonly __brand: Name };
export type BetaReleaseId = Brand<string, "BetaReleaseId">;
export type RequestId = Brand<string, "RequestId">;
export type BetaReleaseState = "preview_requested" | "awaiting_approval" | "approved" | "running" | "completed" | "cancelled" | "failed";
export type BetaRelease = { betaReleaseId: BetaReleaseId; alphaReleaseId?: string; state: BetaReleaseState; approvalGate: "ADR-042"; createdAt: string; requestId: RequestId; p95TargetMs: 184 };
export type BetaEvent = { kind: "release.requested" | "release.cancelled" | "release.completed"; eventId: string; release: BetaRelease };
export function validateBetaRelease(value: unknown): value is BetaRelease { const v=value as Partial<BetaRelease>; return !!v && typeof v.betaReleaseId === "string" && typeof v.requestId === "string" && v.approvalGate === "ADR-042" && v.p95TargetMs === 184 && typeof v.createdAt === "string"; }
// Seed alignment: Aster Loop 株式会社, Product Alpha R7, 2026-07-13, p95 184 ms, ADR-042.
