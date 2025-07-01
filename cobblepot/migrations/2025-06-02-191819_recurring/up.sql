CREATE TABLE recurring_transactions (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    account_type INTEGER NOT NULL,
    amount REAL NOT NULL,
    rrule TEXT NOT NULL,
    start_date TIMESTAMP NOT NULL,
    closed BOOLEAN NOT NULL DEFAULT FALSE,
    account_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account (id)
);
