CREATE TABLE IF NOT EXISTS cv_layout (
  user_id     TEXT PRIMARY KEY,
  layout      TEXT NOT NULL DEFAULT '{"rows":[]}',
  updated_at  TEXT DEFAULT CURRENT_TIMESTAMP
);
