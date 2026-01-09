#!/usr/bin/env python3
"""Copy data from a table in one SQLite database to the same table in another."""

import argparse
import sqlite3
import sys


def copy_table(source_db: str, dest_db: str, table_name: str, clear_dest: bool = False) -> int:
    """
    Copy all rows from a table in the source database to the destination database.

    Args:
        source_db: Path to the source SQLite database
        dest_db: Path to the destination SQLite database
        table_name: Name of the table to copy
        clear_dest: If True, delete existing rows in destination table first

    Returns:
        Number of rows copied
    """
    source_conn = sqlite3.connect(source_db)
    dest_conn = sqlite3.connect(dest_db)

    try:
        source_cursor = source_conn.cursor()
        dest_cursor = dest_conn.cursor()

        # Get column names from source table
        source_cursor.execute(f"PRAGMA table_info({table_name})")
        columns = [row[1] for row in source_cursor.fetchall()]

        if not columns:
            raise ValueError(f"Table '{table_name}' not found in source database")

        # Optionally clear destination table
        if clear_dest:
            dest_cursor.execute(f"DELETE FROM {table_name}")
            print(f"Cleared existing rows from destination table '{table_name}'")

        # Fetch all rows from source
        source_cursor.execute(f"SELECT * FROM {table_name}")
        rows = source_cursor.fetchall()

        if not rows:
            print(f"No rows to copy from table '{table_name}'")
            return 0

        # Insert into destination
        placeholders = ", ".join(["?"] * len(columns))
        column_names = ", ".join(columns)
        insert_sql = f"INSERT INTO {table_name} ({column_names}) VALUES ({placeholders})"

        dest_cursor.executemany(insert_sql, rows)
        dest_conn.commit()

        return len(rows)

    finally:
        source_conn.close()
        dest_conn.close()


def main():
    parser = argparse.ArgumentParser(
        description="Copy data from a table in one SQLite database to another"
    )
    parser.add_argument("source_db", help="Path to source SQLite database")
    parser.add_argument("dest_db", help="Path to destination SQLite database")
    parser.add_argument("table_name", help="Name of the table to copy")
    parser.add_argument(
        "--clear",
        action="store_true",
        help="Clear destination table before copying"
    )

    args = parser.parse_args()

    try:
        count = copy_table(args.source_db, args.dest_db, args.table_name, args.clear)
        print(f"Successfully copied {count} rows from '{args.table_name}'")
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
