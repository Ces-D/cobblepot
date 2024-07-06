CREATE TABLE IF NOT EXISTS transaction_event (
  transaction_code INTEGER PRIMARY KEY NOT NULL,
  amount INTEGER NOT NULL DEFAULT 0,
  currency TEXT NOT NULL,
  created_on DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS journal (
  entry_code INTEGER PRIMARY KEY NOT NULL,
  memo TEXT NOT NULL,
  account_code INTEGER NOT NULL,
  transaction_code INTEGER NOT NULL,
  FOREIGN KEY(account_code) REFERENCES account(account_code),
  FOREIGN KEY(transaction_code) REFERENCES transaction_event(transaction_code)
);

