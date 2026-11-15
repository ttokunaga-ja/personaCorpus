#!/usr/bin/env node
import fs from "node:fs";
const [beforeFile, afterFile] = process.argv.slice(2); if (!beforeFile || !afterFile) throw new Error("usage: schema_diff.mjs before.json after.json");
const before = JSON.parse(fs.readFileSync(beforeFile)), after = JSON.parse(fs.readFileSync(afterFile)); const breaks = [];
for (const [path, oldPath] of Object.entries(before.paths ?? {})) { if (!after.paths?.[path]) breaks.push(`endpoint removed: ${path}`); else for (const method of Object.keys(oldPath)) if (!after.paths[path][method]) breaks.push(`operation removed: ${method.toUpperCase()} ${path}`); }
for (const [name, oldSchema] of Object.entries(before.components?.schemas ?? {})) { const next = after.components?.schemas?.[name]; if (!next) continue; for (const field of next.required ?? []) if (!(oldSchema.required ?? []).includes(field)) breaks.push(`required field added: ${name}.${field}`); for (const [field, oldProp] of Object.entries(oldSchema.properties ?? {})) { const nextEnum = next.properties?.[field]?.enum; if (oldProp.enum && nextEnum && oldProp.enum.some(v => !nextEnum.includes(v))) breaks.push(`enum narrowed: ${name}.${field}`); } }
console.log(JSON.stringify({ compatible: breaks.length === 0, breaks }, null, 2)); process.exitCode = breaks.length ? 1 : 0;
