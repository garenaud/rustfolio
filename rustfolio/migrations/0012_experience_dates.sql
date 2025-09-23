ALTER TABLE experiences ADD COLUMN date_start TEXT NOT NULL DEFAULT '';
ALTER TABLE experiences ADD COLUMN date_end   TEXT NOT NULL DEFAULT '';

CREATE INDEX IF NOT EXISTS idx_experiences_date_start ON experiences(date_start);
CREATE INDEX IF NOT EXISTS idx_experiences_date_end   ON experiences(date_end);
