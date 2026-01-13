#!/usr/bin/env python3
"""
Seed script for generating fake test data for the cobblepot database.

Usage:
    python seed_test_db.py <database-url>

Example:
    python seed_test_db.py sqlite:///test.db
    python seed_test_db.py /path/to/database.db
"""

import argparse
import random
import sqlite3
import time
from datetime import datetime, timedelta


def parse_database_url(url: str) -> str:
    """Extract the file path from a database URL."""
    if url.startswith("sqlite:///"):
        return url[len("sqlite:///"):]
    elif url.startswith("sqlite://"):
        return url[len("sqlite://"):]
    return url


def random_unix_timestamp(start_year: int = 2020, end_year: int = 2025) -> int:
    """Generate a random unix timestamp between start_year and end_year."""
    start = datetime(start_year, 1, 1)
    end = datetime(end_year, 12, 31)
    delta = end - start
    random_days = random.randint(0, delta.days)
    random_date = start + timedelta(days=random_days)
    return int(random_date.timestamp())


def generate_rrule() -> str:
    """Generate a random RRULE string for recurrence."""
    frequencies = ["DAILY", "WEEKLY", "MONTHLY", "YEARLY"]
    freq = random.choice(frequencies)
    interval = random.randint(1, 4)
    count = random.randint(6, 24)
    return f"FREQ={freq};INTERVAL={interval};COUNT={count}"


# Sample data for realistic generation
ACCOUNT_NAMES = [
    "Chase Checking", "Wells Fargo Savings", "Vanguard 401k", "Fidelity IRA",
    "Capital One Credit", "Amex Platinum", "Schwab Brokerage", "Ally Savings",
    "Bank of America Checking", "Discover Credit", "TD Ameritrade", "E*Trade",
    "Marcus Savings", "Citi Double Cash", "US Bank Checking", "PNC Savings"
]

OWNERS = ["John", "Jane", "Joint", "Business", "Trust"]

ACCOUNT_TYPES = [0, 1, 2, 3]  # Checking, Savings, Investment, Credit

BALANCE_MEMOS = [
    "Monthly statement", "Quarterly review", "End of year balance",
    "Account reconciliation", "Transfer adjustment", "Interest posted",
    "Dividend received", "Fee adjustment", "Manual entry", "Auto-sync"
]

BUDGET_NAMES = [
    "Monthly Household", "Q1 Savings Goal", "Vacation Fund", "Emergency Fund",
    "Home Improvement", "Annual Insurance", "Holiday Shopping", "Car Maintenance",
    "Healthcare Reserve", "Education Fund", "Wedding Fund", "Tech Upgrades"
]

BUDGET_ITEM_NAMES = [
    "Rent/Mortgage", "Utilities", "Groceries", "Transportation", "Insurance",
    "Entertainment", "Dining Out", "Subscriptions", "Healthcare", "Clothing",
    "Personal Care", "Gifts", "Home Supplies", "Pet Expenses", "Savings Transfer",
    "Debt Payment", "Phone Bill", "Internet", "Gym Membership", "Childcare"
]


def seed_accounts(cursor: sqlite3.Cursor, count: int = 10) -> list[int]:
    """Insert fake account records and return their IDs."""
    account_ids = []
    used_names = set()

    for i in range(count):
        # Ensure unique names
        name = random.choice(ACCOUNT_NAMES)
        suffix = 1
        original_name = name
        while name in used_names:
            name = f"{original_name} {suffix}"
            suffix += 1
        used_names.add(name)

        description = f"Test account for {name}" if random.random() > 0.3 else None
        owner = random.choice(OWNERS)
        account_type = random.choice(ACCOUNT_TYPES)
        opened_on = random_unix_timestamp(2018, 2023)
        closed_on = random_unix_timestamp(2024, 2025) if random.random() < 0.1 else None

        cursor.execute(
            """INSERT INTO account (name, description, owner, account_type, opened_on, closed_on)
               VALUES (?, ?, ?, ?, ?, ?)""",
            (name, description, owner, account_type, opened_on, closed_on)
        )
        account_ids.append(cursor.lastrowid)

    return account_ids


def seed_balances(cursor: sqlite3.Cursor, account_ids: list[int], count_per_account: int = 5) -> list[int]:
    """Insert fake balance records for each account."""
    balance_ids = []

    for account_id in account_ids:
        # Generate balances over time for this account
        base_amount = random.uniform(1000, 50000)

        for i in range(count_per_account):
            memo = random.choice(BALANCE_MEMOS)
            # Vary the amount slightly from the base
            amount = base_amount + random.uniform(-5000, 10000)
            amount = round(max(0, amount), 2)
            entered_on = random_unix_timestamp(2023, 2025)

            cursor.execute(
                """INSERT INTO balance (memo, amount, entered_on, account_id)
                   VALUES (?, ?, ?, ?)""",
                (memo, amount, entered_on, account_id)
            )
            balance_ids.append(cursor.lastrowid)

    return balance_ids


def seed_budgets(cursor: sqlite3.Cursor, count: int = 5) -> list[int]:
    """Insert fake budget records (without recurrence initially)."""
    budget_ids = []
    used_names = set()

    for i in range(count):
        name = random.choice(BUDGET_NAMES)
        suffix = 1
        original_name = name
        while name in used_names:
            name = f"{original_name} {suffix}"
            suffix += 1
        used_names.add(name)

        description = f"Budget for {name.lower()}" if random.random() > 0.4 else None
        anticipated_amount = round(random.uniform(500, 10000), 2)

        cursor.execute(
            """INSERT INTO budget (name, description, anticipated_amount, budget_recurrence_id)
               VALUES (?, ?, ?, NULL)""",
            (name, description, anticipated_amount)
        )
        budget_ids.append(cursor.lastrowid)

    return budget_ids


def seed_budget_items(cursor: sqlite3.Cursor, budget_ids: list[int], items_per_budget: int = 4) -> list[int]:
    """Insert fake budget item records for each budget."""
    budget_item_ids = []

    for budget_id in budget_ids:
        used_names = set()

        for i in range(items_per_budget):
            name = random.choice(BUDGET_ITEM_NAMES)
            suffix = 1
            original_name = name
            while name in used_names:
                name = f"{original_name} {suffix}"
                suffix += 1
            used_names.add(name)

            description = f"Expense for {name.lower()}" if random.random() > 0.5 else None
            amount = round(random.uniform(50, 2000), 2)

            cursor.execute(
                """INSERT INTO budget_item (name, description, amount, budget_id, budget_recurrence_id)
                   VALUES (?, ?, ?, ?, NULL)""",
                (name, description, amount, budget_id)
            )
            budget_item_ids.append(cursor.lastrowid)

    return budget_item_ids


def seed_budget_item_accounts(
    cursor: sqlite3.Cursor,
    budget_item_ids: list[int],
    account_ids: list[int]
) -> int:
    """Link budget items to accounts with allocation percentages."""
    count = 0

    for budget_item_id in budget_item_ids:
        # Each budget item links to 1-3 accounts
        num_accounts = random.randint(1, min(3, len(account_ids)))
        selected_accounts = random.sample(account_ids, num_accounts)

        # Distribute allocation percentages
        if num_accounts == 1:
            allocations = [100]
        else:
            # Generate random allocations that sum to 100
            allocations = []
            remaining = 100
            for j in range(num_accounts - 1):
                alloc = random.randint(10, remaining - 10 * (num_accounts - j - 1))
                allocations.append(alloc)
                remaining -= alloc
            allocations.append(remaining)

        for account_id, allocation in zip(selected_accounts, allocations):
            try:
                cursor.execute(
                    """INSERT INTO budget_item_account (account_id, budget_item_id, allocation_percentage)
                       VALUES (?, ?, ?)""",
                    (account_id, budget_item_id, allocation)
                )
                count += 1
            except sqlite3.IntegrityError:
                # Skip if this combination already exists
                pass

    return count


def seed_budget_recurrences(
    cursor: sqlite3.Cursor,
    budget_ids: list[int],
    budget_item_ids: list[int],
    budget_recurrence_count: int = 2,
    item_recurrence_count: int = 5
) -> list[int]:
    """Insert budget recurrence records and update related budgets/items."""
    recurrence_ids = []

    # Create recurrences for some budgets
    selected_budgets = random.sample(budget_ids, min(budget_recurrence_count, len(budget_ids)))
    for budget_id in selected_budgets:
        dt_start = random_unix_timestamp(2024, 2025)
        recurrence_rule = generate_rrule()

        cursor.execute(
            """INSERT INTO budget_recurrence (dt_start, recurrence_rule, budget_id, budget_item_id)
               VALUES (?, ?, ?, NULL)""",
            (dt_start, recurrence_rule, budget_id)
        )
        recurrence_id = cursor.lastrowid
        recurrence_ids.append(recurrence_id)

        # Update the budget to reference this recurrence
        cursor.execute(
            """UPDATE budget SET budget_recurrence_id = ? WHERE id = ?""",
            (recurrence_id, budget_id)
        )

    # Create recurrences for some budget items
    selected_items = random.sample(budget_item_ids, min(item_recurrence_count, len(budget_item_ids)))
    for budget_item_id in selected_items:
        dt_start = random_unix_timestamp(2024, 2025)
        recurrence_rule = generate_rrule()

        cursor.execute(
            """INSERT INTO budget_recurrence (dt_start, recurrence_rule, budget_id, budget_item_id)
               VALUES (?, ?, NULL, ?)""",
            (dt_start, recurrence_rule, budget_item_id)
        )
        recurrence_id = cursor.lastrowid
        recurrence_ids.append(recurrence_id)

        # Update the budget item to reference this recurrence
        cursor.execute(
            """UPDATE budget_item SET budget_recurrence_id = ? WHERE id = ?""",
            (recurrence_id, budget_item_id)
        )

    return recurrence_ids


def seed_database(db_path: str, config: dict = None):
    """Main function to seed the database with test data."""
    config = config or {
        "accounts": 10,
        "balances_per_account": 5,
        "budgets": 5,
        "items_per_budget": 4,
        "budget_recurrences": 2,
        "item_recurrences": 5,
    }

    print(f"Connecting to database: {db_path}")
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    # Enable foreign keys
    cursor.execute("PRAGMA foreign_keys = ON")

    try:
        print("Seeding accounts...")
        account_ids = seed_accounts(cursor, config["accounts"])
        print(f"  Created {len(account_ids)} accounts")

        print("Seeding balances...")
        balance_ids = seed_balances(cursor, account_ids, config["balances_per_account"])
        print(f"  Created {len(balance_ids)} balances")

        print("Seeding budgets...")
        budget_ids = seed_budgets(cursor, config["budgets"])
        print(f"  Created {len(budget_ids)} budgets")

        print("Seeding budget items...")
        budget_item_ids = seed_budget_items(cursor, budget_ids, config["items_per_budget"])
        print(f"  Created {len(budget_item_ids)} budget items")

        print("Seeding budget item accounts...")
        bia_count = seed_budget_item_accounts(cursor, budget_item_ids, account_ids)
        print(f"  Created {bia_count} budget-item-account links")

        print("Seeding budget recurrences...")
        recurrence_ids = seed_budget_recurrences(
            cursor, budget_ids, budget_item_ids,
            config["budget_recurrences"], config["item_recurrences"]
        )
        print(f"  Created {len(recurrence_ids)} recurrences")

        conn.commit()
        print("\nDatabase seeded successfully!")

    except Exception as e:
        conn.rollback()
        print(f"Error seeding database: {e}")
        raise
    finally:
        conn.close()


def main():
    parser = argparse.ArgumentParser(
        description="Seed a cobblepot database with fake test data"
    )
    parser.add_argument(
        "database_url",
        help="Path to SQLite database (e.g., sqlite:///test.db or /path/to/db.sqlite)"
    )
    parser.add_argument(
        "--accounts", type=int, default=10,
        help="Number of accounts to create (default: 10)"
    )
    parser.add_argument(
        "--balances-per-account", type=int, default=5,
        help="Number of balances per account (default: 5)"
    )
    parser.add_argument(
        "--budgets", type=int, default=5,
        help="Number of budgets to create (default: 5)"
    )
    parser.add_argument(
        "--items-per-budget", type=int, default=4,
        help="Number of items per budget (default: 4)"
    )
    parser.add_argument(
        "--seed", type=int, default=None,
        help="Random seed for reproducible data generation"
    )

    args = parser.parse_args()

    if args.seed is not None:
        random.seed(args.seed)
        print(f"Using random seed: {args.seed}")

    db_path = parse_database_url(args.database_url)

    config = {
        "accounts": args.accounts,
        "balances_per_account": args.balances_per_account,
        "budgets": args.budgets,
        "items_per_budget": args.items_per_budget,
        "budget_recurrences": 2,
        "item_recurrences": 5,
    }

    seed_database(db_path, config)


if __name__ == "__main__":
    main()
