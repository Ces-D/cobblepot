# Cobblepot

<p align="center">
<img src="https://media.giphy.com/media/UHZMvURcKk8IU/giphy.gif" ></img>
</p>

## Purpose

Cobblepot is a command-line personal finance tool designed to empower users with a streamlined, efficient way to manage their financial data. Its core functions—managing accounts, tracking account balances, and generating insightful reports such as growth over time and balance sheets—provide a centralized platform to monitor your financial health.

The tool’s purpose is to simplify the often complex world of personal finance by offering:

- **Account Management:** Easily add, update, and view multiple financial accounts.
- **Balance Tracking:** Keep an accurate, real-time snapshot of your financial standing.
- **Simple Reporting:** Generate essential reports that visualize your financial growth and current balance sheets.

Inspired by its namesake—the cunning Batman villain—Cobblepot embodies strategic precision and meticulous control. It is built to help you navigate your personal finances with the same level of foresight and determination, ensuring you are always in command of your financial destiny.

## Commands

```bash
A personal finance journal

Usage: cobblepot <COMMAND>

Commands:
  new-account    Add a new account
  edit-account   Edit an existing account
  list-accounts  List Accounts
  new-balance    Add an updated balance entry to an account
  edit-balance   Edit an existing balance entry
  list-balances  List balance entries of an account
  balance-sheet  Calculate a BalanceSheet
  deep-dive      Dive deep into account analytics
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Config

There is one single config parameter: `connection_url`. This is the path to the SQLite database file. If the parameter is not provided then the default path of `~/.local/share/cobblepot.db` will be used. The path may be a little different depending on your operating system.

#### TODO

- `deep-dive` `historical-min-balance` is 0.0 when only one entry is entered. This could be an error in logi since min should be the latest balance
- Add a recurring section. That takes in the interval of charge or income, the start date. Then calculates the monthly required balance to cover charges and total monthly income.
- Add a command to generate a recurrence rule since its difficult to do manually
- connect to google calendar for updates
- create system to add, update, and delte the created events also make these update the local db
