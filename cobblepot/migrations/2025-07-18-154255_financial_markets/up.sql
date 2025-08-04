CREATE TABLE market_instrument(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    ticker TEXT NOT NULL UNIQUE,
    market TEXT,
    instrument_type INTEGER NOT NULL,
    quantity REAL NOT NULL,
    opened_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,

    account_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id)
);

