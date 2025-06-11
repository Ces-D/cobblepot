CREATE TABLE account (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  owner TEXT NOT NULL,
  account_type INTEGER NOT NULL,
  opened_on TEXT NOT NULL,
  closed_on TEXT
);

CREATE TABLE balance (
  id INTEGER PRIMARY KEY NOT NULL,
  memo TEXT NOT NULL,
  amount REAL NOT NULL,
  entered_on TEXT NOT NULL,
  account_id INTEGER NOT NULL,
  FOREIGN KEY (account_id) REFERENCES account (id)
);

