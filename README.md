# Cobblepot

<p align="center">
<img src="https://media.giphy.com/media/UHZMvURcKk8IU/giphy.gif" ></img>
</p>

A lightweight, self-hosted personal finance tool that runs locally to help you track accounts, balances, and recurring transactions—then generates deep-dive reports and balance sheets so you always know where your money is going.

## 🚀 Key Features

- **Account Management**
  Create, update, and archive multiple accounts (checking, savings, credit, crypto, etc.).
- **Real-Time Balances**
  View and add current balances for any account at any time.
- **Automated Recurring Transactions**
  Set up trackers for your regular income (paychecks, interest) and expenses (rent, subscriptions) just once—Cobblepot then manually applies each scheduled entry to your balances, so you can monitor their impact and never miss a cycle.
- **Deep-Dive Reporting**
  Generate:

  - **Account History**: transaction-by-transaction breakdown
  - **Balance Sheets**: snapshot of assets & liabilities over time

- **Local Server & Database**
  All data stays on your machine (SQLite by default), no external dependencies or cloud lock-in.
- **Flexible Interface**

  - **REST API** to build your own dashboards or integrations

Inspired by its namesake—the cunning Batman villain—Cobblepot embodies strategic precision and meticulous control. It is built to help you navigate your personal finances with the same level of foresight and determination, ensuring you are always in command of your financial destiny.

## 📦 Installation

1. **Clone the repo**

   ```bash
   git clone git@github.com:Ces-D/cobblepot.git
   cd cobblepot
   ```

2. **Configure**
   Set the optional environment variable `COBBLEPOT_DB_NAME`

## 🤝 Contributing

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/awesome`)
3. Commit your changes (`git commit -m "Add awesome feature"`)
4. Push to your branch (`git push origin feature/awesome`)
5. Open a Pull Request

Please adhere to the existing code style and write tests for new functionality.

## 📄 License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
