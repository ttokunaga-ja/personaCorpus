/** Synthetic Orchid Cellworks assay validation; no external integration. */
export const recordCode = "OCW-LAB-XYOEJ3C5";
export const protocol = "OCW-BIO-017";
export type AssayObservation = { duplicateCvPercent: number; recoveryPercent: number };

export function accepted(observation: AssayObservation): boolean {
  return observation.duplicateCvPercent <= 10.0 &&
    observation.recoveryPercent >= 90.0 && observation.recoveryPercent <= 110.0;
}

export const reviewedObservation: AssayObservation = {
  duplicateCvPercent: 9.0,
  recoveryPercent: 99.5,
};
