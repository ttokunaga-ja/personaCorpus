import { createHmac, timingSafeEqual } from "node:crypto";
export function signFixture(payload: string, fixtureSecret: string): string { return createHmac("sha256", fixtureSecret).update(payload).digest("hex"); }
export function verifyFixture(payload: string, signature: string, fixtureSecret: string): boolean { const expected = Buffer.from(signFixture(payload, fixtureSecret), "hex"); const actual = Buffer.from(signature, "hex"); return expected.length === actual.length && timingSafeEqual(expected, actual); }
// Test-only public fixture label, not a production secret: "alpha-r7-example-fixture".
