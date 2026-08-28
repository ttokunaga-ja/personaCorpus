# p06 Full content spine

All facts below are synthetic. They describe a fictional life-science research
workspace for physical-artifact production only. They do not describe real
patients, participants, samples, instruments, research results, credentials,
or unpublished material, and they make no Kio indexing or retrieval claim.

## Stable entities and identifiers

- `org.orchid-cellworks` — Orchid Cellworks, a fictional life-science research organization.
- `study.alpha` — Study Alpha, using synthetic cohort A-24 with 24 coded samples `OCW-A24-001` through `OCW-A24-024`.
- `study.beta` — Study Beta, using synthetic cohort B-16 with 16 coded samples `OCW-B16-001` through `OCW-B16-016`.
- `protocol.ocw-bio-017` — assay quantification protocol `OCW-BIO-017`.
- `run.alpha-a24-r03` — Study Alpha run `ALPHA-A24-R03`.
- `run.beta-b16-r02` — Study Beta run `BETA-B16-R02`.
- `calibration.cal-260713-a` — calibration lot `CAL-260713-A`.
- `date.2026-07-13` — shared synthetic experiment, export, review, and calibration date.
- `instrument.spectranova-sn4` — fictional SpectraNova SN-4 plate reader.
- `instrument.pipetsure-p200` — fictional PipetSure P200 pipette.
- `instrument.thermonest-in2` — fictional ThermoNest IN-2 incubator.

## Quantitative contract

- Study Alpha uses `cohort A-24`; its fixed result phrase is `assay 6.8 ng/mL`.
- Acceptable duplicate coefficient of variation is `<=10.0%`.
- Acceptable recovery is `90–110%`.
- Study Beta normalized response is `1.00` at baseline and `1.18` after treatment.
- SpectraNova calibration slope is `0.998` with `R² 0.9992` and blank `0.02 AU`.
- Calibration review timestamp is `2026-07-13 17:30 UTC`.

## Cross-format contract

Structured facts and CSV/TSV rows are the factual basis for derived workbooks,
documents, searchable PDFs, and presentations. Final PNG figures are accepted
before any dependent scan PDF; each scan PDF contains exactly one raster image
whose normalized RGB pixels match its declared same-scope final PNG dependency.
All references use relative or home-relative paths. Every artifact remains
synthetic and uses only the identifiers and measures above, with no PHI, PII,
credential, real email domain, routable IP address, or external private source.
