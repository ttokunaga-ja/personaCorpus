/** Synthetic Orchid Cellworks assay validation; no external integration. */
export const recordCode = "OCW-LAB-WQXDTGMW";
export const protocol = "OCW-BIO-017";
export type AssayObservation = { duplicateCvPercent: number; recoveryPercent: number };

export function accepted(observation: AssayObservation): boolean {
  return observation.duplicateCvPercent <= 10.0 &&
    observation.recoveryPercent >= 90.0 && observation.recoveryPercent <= 110.0;
}

export const reviewedObservation: AssayObservation = {
  duplicateCvPercent: 7.9,
  recoveryPercent: 107.3,
};
