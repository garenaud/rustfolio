CREATE TABLE IF NOT EXISTS project_technologies (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  tech TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_project_tech_pid ON project_technologies(project_id);
