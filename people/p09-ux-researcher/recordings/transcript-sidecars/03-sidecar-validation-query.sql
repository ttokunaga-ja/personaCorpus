-- structured_text family: SQLite validation fixture for Beacon Field Research.
-- All hashes are explicitly synthetic fixture values, not hashes of delivered files.
CREATE TABLE IF NOT EXISTS sidecar_fixture (
  session_id TEXT PRIMARY KEY,
  organization TEXT NOT NULL,
  study TEXT NOT NULL,
  study_date TEXT NOT NULL,
  sample_size INTEGER NOT NULL CHECK (sample_size = 24),
  export_id TEXT NOT NULL,
  recorder_fixture_id TEXT NOT NULL,
  sidecar_id TEXT NOT NULL,
  recorder_hash_fixture TEXT NOT NULL,
  sidecar_hash_fixture TEXT NOT NULL,
  created_at TEXT NOT NULL
);

INSERT INTO sidecar_fixture VALUES (
  'UX-184', 'Beacon Field Research', 'Study Alpha', '2026-07-13', 24,
  'EXP-UX184-01', 'REC-UX184-A', 'SC-UX184-01',
  '7f429455634d70d0c35f3cfb0f203f8cb2edec0ea71e7a49e5727a4f4608d45c',
  'e91e1bc8c0a9bb0e0d7f6ed5de3ceff165bf2d6f35cb8a4f3fb3b08eec542bf9',
  '2026-07-13T10:10:00+09:00'
);

SELECT session_id, export_id, recorder_fixture_id, sidecar_id
FROM sidecar_fixture
WHERE organization = 'Beacon Field Research'
  AND study = 'Study Alpha'
  AND study_date = '2026-07-13'
  AND sample_size = 24
  AND session_id = 'UX-184';
