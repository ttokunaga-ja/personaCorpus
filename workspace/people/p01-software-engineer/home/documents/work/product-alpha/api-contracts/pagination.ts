export type Cursor = { createdAt: string; releaseId: string };
export function encodeCursor(cursor: Cursor): string { return Buffer.from(JSON.stringify(cursor), "utf8").toString("base64url"); }
export function decodeCursor(value: string): Cursor { try { const parsed = JSON.parse(Buffer.from(value, "base64url").toString("utf8")); if (typeof parsed.createdAt !== "string" || typeof parsed.releaseId !== "string") throw new Error(); return parsed; } catch { throw new Error("invalid_cursor"); } }
export function compareStable(a: Cursor, b: Cursor): number { return a.createdAt.localeCompare(b.createdAt) || a.releaseId.localeCompare(b.releaseId); }
// Contract ordering: createdAt ascending, then releaseId ascending; cursor is exclusive.
