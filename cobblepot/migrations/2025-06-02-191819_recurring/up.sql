CREATE TABLE recurring_transactions (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    account_type INTEGER NOT NULL,
    amount REAL NOT NULL,
    rrule TEXT NOT NULL,
    status INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account (id)
);
