# Cobblepot

<p align="center">
<img src="https://media.giphy.com/media/UHZMvURcKk8IU/giphy.gif" ></img>
</p>

<p align="center">
  <strong>A self-hosted personal finance CLI for tracking accounts, balances, budgets, and reports</strong>
</p>

---

## Overview

Cobblepot is a powerful command-line tool written in Rust for managing your personal finances. It provides a simple, self-hosted solution to track accounts, balances, budgets, and generate financial reports—all stored locally in a SQLite database.

## Features

- **Account Management**: Track multiple financial accounts (checking, savings, credit cards, etc.)
- **Balance Tracking**: Record and monitor account balances over time
- **Budget Planning**: Create budgets with recurring rules and track budget items across accounts
- **Financial Reports**: Generate balance sheets and analyze spending patterns
- **Self-Hosted**: All data stored locally in SQLite—you own your financial data
- **Fast & Lightweight**: Built with Rust for speed and reliability
- **Financial Market Integration**: Tiingo API integration for market data (optional)

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- SQLite 3.35+
- Diesel CLI

### From Source

```bash
git clone https://github.com/yourusername/cobblepot.git
cd cobblepot
cargo build --release
```

The binary will be available at `target/release/cobblepot`.

### Environment Variables

## Usage

```bash
# List all accounts
cobblepot list accounts

# Create a new account
cobblepot create account --name "Checking" --type checking --owner "John Doe"

# Add a balance entry
cobblepot create balance --account-id 1 --amount 5000.00 --memo "Opening balance"

# Generate a balance sheet report
cobblepot report balance-sheet
```

## Commands

### `list`

Display accounts, balances, budgets, or budget items

```bash
cobblepot list accounts
cobblepot list balances
cobblepot list budgets
cobblepot list budget-items
```

### `create`

Create new accounts, balances, budgets, or budget items

```bash
cobblepot create account --name "Savings" --type savings --owner "Jane Doe"
cobblepot create balance --account-id 2 --amount 10000 --memo "Initial deposit"
cobblepot create budget --name "Monthly Budget" --amount 3000
```

### `report`

Generate financial reports

```bash
cobblepot report balance-sheet
```

## Architecture

Cobblepot is built with a modular workspace structure:

- **`cli/`**: Command-line interface and user interaction layer
- **`core/`**: Core business logic and configuration
- **`data_store/`**: Database schema, migrations, and data access layer (Diesel ORM)
- **`financial_markets/`**: Integration with financial market APIs (Tiingo)

### Data Model

- **Accounts**: Financial accounts with types (checking, savings, credit, etc.)
- **Balances**: Historical balance entries linked to accounts
- **Budgets**: Budget definitions with recurring rules
- **Budget Items**: Individual line items within budgets
- **Budget Item Accounts**: Allocation of budget items across accounts

## Development

### Running Tests

```bash
cargo test --workspace
```

### Database Migrations

```bash
cd data_store
diesel migration run
```

## License

This project is open source. Please refer to the license file for details.

## Author

Cesar Diaz

