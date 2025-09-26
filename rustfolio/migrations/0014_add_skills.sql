-- migrations/XXXX_add_skills.sql
CREATE TABLE IF NOT EXISTS skills (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id    INTEGER NOT NULL,
  name       TEXT    NOT NULL,
  percentage INTEGER NULL CHECK (percentage BETWEEN 0 AND 100),
  logo_url   TEXT    NULL,
  category   TEXT    NOT NULL DEFAULT 'General',
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_skills_user ON skills(user_id);
CREATE INDEX IF NOT EXISTS idx_skills_user_cat ON skills(user_id, category);
