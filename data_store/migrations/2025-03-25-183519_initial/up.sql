CREATE TABLE IF NOT EXISTS account (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    owner TEXT NOT NULL,
    account_type INTEGER NOT NULL,
    opened_on INTEGER NOT NULL,
    closed_on INTEGER 
);

CREATE TABLE IF NOT EXISTS balance (
    id INTEGER PRIMARY KEY NOT NULL,
    memo TEXT NOT NULL,
    amount REAL NOT NULL,
    entered_on INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account (id) ON DELETE CASCADE
);

