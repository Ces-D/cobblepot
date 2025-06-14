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

### `cobblepot --help`

```bash
Cobblepot is a command-line personal finance tool designed to empower users with a streamlined, efficient way to manage their financial data

Usage: cobblepot <COMMAND>

Commands:
  open    Open a new entity
  update  Update an existing entity
  close   Close an existing entity
  apply   Apply an effect to balances
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
 ### `cobblepot open --help`

```bash
Open a new entity

Usage: cobblepot open <COMMAND>

Commands:
  account    Open a new account
  balance    Open a new balance
  recurring  Open a new recurring entity
  report     Generate a report
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `cobblepot update --help`

```bash
Update an existing entity

Usage: cobblepot update <COMMAND>

Commands:
  account  Update an existing account
  balance  Update an existing balance
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `cobblepot close --help`

```bash
Close an existing entity

Usage: cobblepot close <COMMAND>

Commands:
  account    Close an existing account
  recurring  Close an existing recurring entity
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Config

There is one single config parameter: `connection_url`. This is the path to the SQLite database file. If the parameter is not provided then the default path of `~/.local/share/cobblepot.db` will be used. The path may be a little different depending on your operating system.

#### TODO

- Add a recurring apply command. To apply any recurring transactions that affected an account balance since the last time it was applied
- connect to google calendar for updates
- create system to add, update, and delete the created events also make these update the local db
- Improve the display of the json dates
