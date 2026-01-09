CREATE TABLE IF NOT EXISTS budget (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  anticipated_amount REAL NOT NULL,
  budget_recurrence_id INTEGER,
  FOREIGN KEY (budget_recurrence_id) REFERENCES budget_recurrence (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS budget_item (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  amount REAL NOT NULL,
  budget_id INTEGER NOT NULL,
  budget_recurrence_id INTEGER,
  FOREIGN KEY (budget_id) REFERENCES budget (id) ON DELETE CASCADE,
  FOREIGN KEY (budget_recurrence_id) REFERENCES budget_recurrence (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS budget_item_account (
  account_id INTEGER NOT NULL,
  budget_item_id INTEGER NOT NULL,
  allocation_percentage INTEGER NULL DEFAULT 100, 
  CHECK (allocation_percentage BETWEEN 0 AND 100),
  FOREIGN KEY (budget_item_id) REFERENCES budget_item (id) ON DELETE CASCADE,
  FOREIGN KEY (account_id) REFERENCES account (id) on DELETE RESTRICT,
  PRIMARY KEY (account_id, budget_item_id)
);

CREATE TABLE IF NOT EXISTS budget_recurrence (
  id INTEGER PRIMARY KEY NOT NULL,
  dt_start INTEGER NOT NULL,
  recurrence_rule TEXT NOT NULL,
  budget_id INTEGER,
  budget_item_id INTEGER,
  FOREIGN KEY (budget_id) REFERENCES budget (id) ON DELETE CASCADE,
  FOREIGN KEY (budget_item_id) REFERENCES budget_item (id) ON DELETE CASCADE,
  CHECK (
      (budget_id IS NOT NULL AND budget_item_id IS NULL) OR
      (budget_id IS NULL AND budget_item_id IS NOT NULL)
  )
)

