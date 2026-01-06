CREATE TABLE budget (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  anticipated_amount REAL NOT NULL,
  starts_on INTEGER NOT NULL,
  ends_on INTEGER,
  recurrence_rule TEXT
);

CREATE TABLE budget_item (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  amount REAL NOT NULL,
  budget_id INTEGER NOT NULL,
  FOREIGN KEY (budget_id) REFERENCES budget (id) ON DELETE CASCADE
);

CREATE TABLE budget_item_account (
  account_id INTEGER NOT NULL,
  budget_item_id INTEGER NOT NULL,
  allocation_percentage INTEGER NULL DEFAULT 100, 
  CHECK (allocation_percentage BETWEEN 0 AND 100),
  FOREIGN KEY (budget_item_id) REFERENCES budget_item (id) ON DELETE CASCADE,
  FOREIGN KEY (account_id) REFERENCES account (id) on DELETE RESTRICT,
  PRIMARY KEY (account_id, budget_item_id)
);

