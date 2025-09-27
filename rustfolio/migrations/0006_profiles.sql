CREATE TABLE IF NOT EXISTS profiles (
  user_id TEXT PRIMARY KEY,
  first_name TEXT DEFAULT '',
  last_name  TEXT DEFAULT '',
  title      TEXT DEFAULT '',
  email      TEXT DEFAULT '',
  phone      TEXT DEFAULT '',
  address    TEXT DEFAULT '',
  city       TEXT DEFAULT '',
  country    TEXT DEFAULT '',
  website    TEXT DEFAULT '',
  photo_url  TEXT DEFAULT '',
  updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
