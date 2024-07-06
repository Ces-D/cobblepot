CREATE TABLE IF NOT EXISTS balance (
  id INTEGER PRIMARY KEY NOT NULL,
  account_code INTEGER NOT NULL,
  current_balance INTEGER NOT NULL,
  journal_entry INTEGER NOT NULL,
  FOREIGN KEY(account_code) REFERENCES account(account_code),
  FOREIGN KEY(current_balance) REFERENCES transaction_event(transaction_code),
  FOREIGN KEY(journal_entry) REFERENCES journal(entry_code)
);
