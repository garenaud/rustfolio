CREATE TABLE IF NOT EXISTS experience_tasks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  experience_id INTEGER NOT NULL REFERENCES experiences(id) ON DELETE CASCADE,
  task TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_tasks_exp ON experience_tasks(experience_id);
