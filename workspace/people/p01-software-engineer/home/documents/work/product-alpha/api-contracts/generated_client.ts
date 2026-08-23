/** Generated-style client for Aster Loop Product Alpha R7 (2026-07-13). */
export type ReleaseState = "requested" | "awaiting_approval" | "approved" | "running" | "completed" | "failed";
export interface Release { releaseId: string; product: "product-alpha"; version: string; state: ReleaseState; createdAt: string; updatedAt: string; approvalGate: "ADR-042"; requestId: string; }
export interface ApiError { code: string; message: string; requestId: string; retryable: boolean; details?: Record<string, string>; }
export interface Page<T> { items: T[]; nextCursor?: string; requestId: string; }
export interface CreateReleaseInput { version: string; changeSummary: string; }

export class AlphaR7Client {
  constructor(private readonly baseUrl: string, private readonly fetcher: typeof fetch = fetch) {}
  async createRelease(input: CreateReleaseInput, idempotencyKey: string, requestId = crypto.randomUUID()): Promise<Release> {
    return this.request<Release>("/v1/releases", { method: "POST", headers: { "content-type": "application/json", "idempotency-key": idempotencyKey, "x-request-id": requestId }, body: JSON.stringify(input) });
  }
  async getRelease(releaseId: string, requestId = crypto.randomUUID()): Promise<Release> { return this.request<Release>(`/v1/releases/${encodeURIComponent(releaseId)}`, { headers: { "x-request-id": requestId } }); }
  async listReleases(cursor?: string, limit = 25): Promise<Page<Release>> { const q = new URLSearchParams({ limit: String(limit) }); if (cursor) q.set("cursor", cursor); return this.request<Page<Release>>(`/v1/releases?${q}`); }
  private async request<T>(path: string, init: RequestInit = {}, attempt = 0): Promise<T> {
    const response = await this.fetcher(new URL(path, this.baseUrl), init); const requestId = response.headers.get("x-request-id") ?? "unknown";
    if (response.status >= 500 && attempt < 2 && init.method !== "POST") return this.request<T>(path, init, attempt + 1);
    const body = await response.json() as T | ApiError; if (!response.ok) throw Object.assign(new Error((body as ApiError).message), { apiError: body, requestId }); return body as T;
  }
}
// Example: await client.createRelease({ version: "r7.0", changeSummary: "Async gate rollout" }, "demo-idempotency-key-20260713");
