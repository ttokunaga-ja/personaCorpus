/** Reject expensive or mutable queries before the local synthetic runner sees them. */
export function validateQuery(query: string, maxCharacters = 2000): string {
  const normalized = query.trim();
  if (!normalized.toLowerCase().startsWith("select") && !normalized.toLowerCase().startsWith("with")) {
    throw new Error("MTR-09 sandbox accepts SELECT or WITH only");
  }
  if (normalized.length > maxCharacters) throw new Error("query exceeds local review budget");
  if (/\b(insert|update|delete|drop|alter|attach)\b/i.test(normalized)) throw new Error("mutable statement blocked");
  return normalized;
}

console.assert(validateQuery("SELECT 84200000 AS gmv_yen") === "SELECT 84200000 AS gmv_yen");
