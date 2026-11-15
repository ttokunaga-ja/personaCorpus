type StudyAlphaExport = {
  organization: string;
  study: string;
  cohort: string;
  baselineDate: string;
  assayValueNgMl: number;
  workstation: string;
};

export type ValidationResult =
  | { ok: true; normalized: StudyAlphaExport }
  | { ok: false; errors: string[] };

const expected = {
  organization: "Orchid Cellworks",
  study: "Study Alpha",
  cohort: "A-24",
  baselineDate: "2026-07-13",
  assayValueNgMl: 6.8,
} as const;

export function validateSyntheticExport(input: unknown): ValidationResult {
  if (typeof input !== "object" || input === null || Array.isArray(input)) {
    return { ok: false, errors: ["Export must be a record."] };
  }
  const record = input as Record<string, unknown>;
  const errors: string[] = [];
  for (const [key, value] of Object.entries(expected)) {
    if (record[key] !== value) errors.push(`Unexpected ${key}.`);
  }
  if (typeof record.workstation !== "string" || !record.workstation.includes("Windows")) {
    errors.push("Workstation must identify the Windows laboratory context.");
  }
  if (errors.length > 0) return { ok: false, errors };
  return {
    ok: true,
    normalized: {
      organization: expected.organization,
      study: expected.study,
      cohort: expected.cohort,
      baselineDate: expected.baselineDate,
      assayValueNgMl: expected.assayValueNgMl,
      workstation: record.workstation as string,
    },
  };
}

const demo: StudyAlphaExport = {
  ...expected,
  workstation: "Windows laboratory workstation (en-US)",
};
const result = validateSyntheticExport(demo);
if (!result.ok) throw new Error(result.errors.join(" "));
console.log(`Validated ${result.normalized.study} ${result.normalized.cohort}: ${result.normalized.assayValueNgMl} ng/mL`);
