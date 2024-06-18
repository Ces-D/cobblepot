CREATE TABLE IF NOT EXISTS account (
  account_code INTEGER PRIMARY KEY NOT NULL,
  account_variant TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  created_on TEXT NOT NULL,
  closed_on TEXT
);

