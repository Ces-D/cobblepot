#!/usr/bin/env python3
"""
Migrate data from an old SQLite database to a new one, converting date strings to UTC timestamps.

Date format: "2025-04-14 19:00:42.116125456" -> Unix timestamp (integer)
"""

import sqlite3
import argparse
from datetime import datetime, timezone
from typing import Optional


def parse_date_to_timestamp(date_str: Optional[str]) -> Optional[int]:
    """Convert date string to UTC timestamp (seconds since epoch)."""
    if date_str is None:
        return None

    # Handle the format: "2025-04-14 19:00:42.116125456"
    # Python's strptime only supports up to 6 decimal places for microseconds
    # Truncate nanoseconds to microseconds
    if "." in date_str:
        base, frac = date_str.rsplit(".", 1)
        frac = frac[:6]  # Truncate to microseconds
        date_str = f"{base}.{frac}"
        dt = datetime.strptime(date_str, "%Y-%m-%d %H:%M:%S.%f")
    else:
        dt = datetime.strptime(date_str, "%Y-%m-%d %H:%M:%S")

    # Assume the date is in UTC and convert to timestamp
    dt = dt.replace(tzinfo=timezone.utc)
    return int(dt.timestamp())


def migrate_account(source_db: str, target_db: str):
    """Migrate account table data."""
    source_conn = sqlite3.connect(source_db)
    target_conn = sqlite3.connect(target_db)

    source_cur = source_conn.cursor()
    target_cur = target_conn.cursor()

    # Fetch all rows from source
    source_cur.execute(
        "SELECT id, name, description, owner, account_type, opened_on, closed_on FROM account"
    )
    rows = source_cur.fetchall()

    migrated = 0
    for row in rows:
        id_, name, description, owner, account_type, opened_on, closed_on = row

        # Convert date fields
        opened_on_ts = parse_date_to_timestamp(opened_on)
        closed_on_ts = parse_date_to_timestamp(closed_on)

        target_cur.execute(
            "INSERT INTO account (id, name, description, owner, account_type, opened_on, closed_on) VALUES (?, ?, ?, ?, ?, ?, ?)",
            (id_, name, description, owner, account_type, opened_on_ts, closed_on_ts),
        )
        migrated += 1

    target_conn.commit()
    source_conn.close()
    target_conn.close()

    print(f"Migrated {migrated} rows from account table")


def migrate_balance(source_db: str, target_db: str):
    """Migrate balance table data."""
    source_conn = sqlite3.connect(source_db)
    target_conn = sqlite3.connect(target_db)

    source_cur = source_conn.cursor()
    target_cur = target_conn.cursor()

    # Fetch all rows from source
    source_cur.execute("SELECT id, memo, amount, entered_on, account_id FROM balance")
    rows = source_cur.fetchall()

    migrated = 0
    for row in rows:
        id_, memo, amount, entered_on, account_id = row

        # Convert date field
        entered_on_ts = parse_date_to_timestamp(entered_on)

        target_cur.execute(
            "INSERT INTO balance (id, memo, amount, entered_on, account_id) VALUES (?, ?, ?, ?, ?)",
            (id_, memo, amount, entered_on_ts, account_id),
        )
        migrated += 1

    target_conn.commit()
    source_conn.close()
    target_conn.close()

    print(f"Migrated {migrated} rows from balance table")


TABLES = {
    "account": migrate_account,
    "balance": migrate_balance,
}


def main():
    parser = argparse.ArgumentParser(
        description="Migrate data from old SQLite database to new one with date conversion"
    )
    parser.add_argument("source_db", help="Path to source SQLite database")
    parser.add_argument("target_db", help="Path to target SQLite database")
    parser.add_argument("table", choices=list(TABLES.keys()), help="Table to migrate")

    args = parser.parse_args()

    migrate_func = TABLES[args.table]
    migrate_func(args.source_db, args.target_db)


if __name__ == "__main__":
    main()
