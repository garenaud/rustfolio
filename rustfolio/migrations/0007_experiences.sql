CREATE TABLE IF NOT EXISTS experiences (
  id       INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id  TEXT NOT NULL,
  date     TEXT DEFAULT '',
  kind     TEXT DEFAULT 'work',     -- ex: work/school/other
  title    TEXT DEFAULT '',
  company  TEXT DEFAULT '',
  location TEXT DEFAULT '',
  updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_experiences_user ON experiences(user_id);
