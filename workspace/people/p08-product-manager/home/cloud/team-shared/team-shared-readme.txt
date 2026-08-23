Juniper Works / Product Alpha Q3
Team-shared roadmap package — as of 2026-07-13

This folder is a synthetic planning handoff for 1,240 synthetic workspaces.
It aligns the roadmap presentation, YAML configuration, and offline widget model.

Shared target
  Adoption: 18.4% baseline -> 26.0% Q3 target (+7.6 percentage points)
  Decision milestone: M-27 — 2026-09-18

Milestone sequence and role owners
  M-24 | 2026-07-24 | Discovery closeout | complete | Product Management
  M-25 | 2026-08-07 | Design and instrumentation ready | on_track | Design + Data
  M-26 | 2026-08-28 | Beta cohort enabled | at_risk | Engineering + Customer Success
  M-27 | 2026-09-18 | Q3 adoption decision | planned | Product Management

Dependency IDs
  DEP-01 Event naming review -> M-25 (Data, on_track)
  DEP-02 Entitlement service capacity confirmation -> M-26 (Engineering, at_risk)
  DEP-03 Beta onboarding playbook -> M-26 (Customer Success, on_track)
  DEP-04 Adoption metric readout -> M-27 (Product Management, planned)

Files
  roadmap-config.yaml         Machine-readable roadmap schema and dependency register
  roadmap-status-widget.ts    Typed, offline status-to-render-model transformation
  cross-functional-roadmap.pptx  Cross-functional review deck

No live system, customer, credential, or network dependency is required.
