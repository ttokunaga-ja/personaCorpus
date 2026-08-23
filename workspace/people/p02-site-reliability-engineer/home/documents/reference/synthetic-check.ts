/** A dependency-free HTTP synthetic check for approved public HTTPS targets. */

export interface SyntheticCheckConfig {
  name: string;
  url: string;
  timeoutMs: number;
  expectedStatus?: number;
}

export interface SyntheticCheckResult {
  name: string;
  ok: boolean;
  status?: number;
  durationMs: number;
  detail: string;
}

function assertSafePublicHttpsUrl(value: string): URL {
  const url = new URL(value);
  const hostname = url.hostname.toLowerCase();
  const disallowed = hostname === "localhost" || hostname.endsWith(".local") ||
    /^127\./.test(hostname) || /^10\./.test(hostname) || /^192\.168\./.test(hostname) ||
    /^172\.(1[6-9]|2\d|3[0-1])\./.test(hostname) || hostname === "::1";
  if (url.protocol !== "https:" || disallowed || url.username || url.password) {
    throw new Error("synthetic checks require a public HTTPS URL without credentials");
  }
  return url;
}

export async function runSyntheticCheck(config: SyntheticCheckConfig): Promise<SyntheticCheckResult> {
  const url = assertSafePublicHttpsUrl(config.url);
  if (!Number.isInteger(config.timeoutMs) || config.timeoutMs < 250 || config.timeoutMs > 30_000) {
    throw new Error("timeoutMs must be an integer between 250 and 30000");
  }

  const started = performance.now();
  const response = await fetch(url, {
    method: "GET",
    redirect: "error",
    signal: AbortSignal.timeout(config.timeoutMs),
    headers: { "accept": "application/json, text/plain;q=0.9, */*;q=0.1" },
  });
  const durationMs = Math.round(performance.now() - started);
  const expectedStatus = config.expectedStatus ?? 200;
  return {
    name: config.name,
    ok: response.status === expectedStatus,
    status: response.status,
    durationMs,
    detail: response.status === expectedStatus ? "status matched" : `expected ${expectedStatus}, received ${response.status}`,
  };
}
