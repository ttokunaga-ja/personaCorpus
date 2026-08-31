# p05 Full content spine

Schema: `persona-corpus.full-content-spine/v1`

Status: **FROZEN before Full physical production**. All organizations, people,
sales figures, systems, requests, and identifiers in p05 are wholly synthetic.
This record contains no credentials, release tokens, real PII/PHI, customer
data, routable endpoints, or Kio/search/history claims.

## Fixed reporting context

- Organization: `Mosaic Retail Analytics`
- Reporting period: `FY26 Q3`
- Reference date: `2026-07-13`
- Metric: `MTR-09`
- Gross recognized merchandise value: `¥86.0M`
- Recognized refunds: `¥1.8M`
- Reported GMV: `¥84.2M`
- Plan: `¥80.0M`
- Plan variance: `+¥4.2M` / `+5.25%`
- Prior comparable period: `¥78.6M`
- Online channel: `¥46.3M`
- Stores channel: `¥37.9M`
- Warehouse refresh through: `2026-07-13 23:59 JST`
- Quality review: `2026-07-14 06:00 JST`

## Stable content identifiers

- `org.mosaic-retail-analytics`
- `period.fy26-q3`
- `date.2026-07-13`
- `metric.mtr-09`
- `measure.gross-jpy-86m`
- `measure.refunds-jpy-1_8m`
- `measure.gmv-jpy-84_2m`
- `measure.plan-jpy-80m`
- `measure.plan-variance-jpy-4_2m-pct-5_25`
- `measure.prior-jpy-78_6m`
- `channel.online-jpy-46_3m`
- `channel.stores-jpy-37_9m`
- `refresh.2026-07-13T23-59-jst`
- `review.2026-07-14T06-00-jst`
- `classification.s2-synthetic-only`
- `claim.no-kio-history-search`

## Dependency order

Within each scope, factual CSV/TSV and structured records precede analytical
XLSX, DOCX, searchable PDF, and PPTX artifacts. Final ImageGen PNG artifacts
follow accepted facts; each scan PDF follows and depends on exactly one earlier
accepted final PNG in the same scope, with normalized RGB pixel identity.
