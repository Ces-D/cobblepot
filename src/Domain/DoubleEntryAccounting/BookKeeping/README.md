# BookKeeping Bounded Context

## Index

### Chart of Accounts

- **Chart of Accounts** - _Index_ of all the financial accounts in the _General Ledger_ of a company broken down into subcategories. Accounts cannot be removed, only added.
  - **Account** - Typically contains name, and an _identification code_, _financial statement_, _group_, _to increase_
  - **Identification Code** - A value used to identify the account.Space is left for additional codes to be inserted in a group. One system for creating this identification code _division code_, _department code_, _account code_. Small businesses and personal may only use the _account code_. This system for creating the identification code must be consistent across _accounting periods_
  - **Financial Statement** - The financial statement in which the account appears. choices include _Balance Sheet_ for asset-liability and equity accounts, _Income Statement_ for revenue and expense accounts,
  - **Group** - Refers to the classification of the account into one of the headings : _Assets_, _Liabilities_, _Shareholder's Equity_, _Revenue_, and _Expenses_.
  - **Sub Group** - The Sub Group column divides each group into the classification needed to produce the _Balance Sheet_ and _Income Statement_ for accounting reports.
    - **Asset Accounts** - The lowest range of values, starting with the current assets, then long-term assets. The Current Asset sub groups can include cash and cash equivalents, accounts receivable, inventory, and other current assets. The Long Term sub groups can include property, plant, and equipment.
    - **Liability Accounts** - The second lowest range of values, starting with current liabilities then long term liabilities. The Current Liabilities sub groups can include accounts payable, and other current liabilities. The Long Term Liabilities can include mortgages and loans.
    - **Equity Accounts** - The third lowest range of values. The sub groups can include capital, and retained earnings.
    - **Income Accounts** - The second largest range of values. The sub groups can include revenue, and other income
    - **Expense Accounts** - The largest range of values. The sub groups can include research and development, sales and marketing, general and administrative, depreciation, finance costs, and income tax expense.
  - **To Increase** - This column is for indicating whether the account is normally increased by a debit or a credit
  - This might be part of the General Ledger Domain

### General Ledger

- **Ledger** - _Record_ of all past _transactions_, organized by _accounts_. Sorts all transaction information through the accounts. Typically there are three different kinds of _Ledgers_ that can be prepared.

  - **Debtors Ledger or Sales Ledgers** - It records all the transactions that take place between you and your debtors, where debtors are whomever you have sold goods that you manufacture. It contains a collection of accounts associated with your customers, and shows the amount owed to you or yet to be received. The following details are recorded: _date of sales_, _type of goods sold_, _amount of goods_, _name of the customers_, _tax details_.
  - **Creditors Ledger or Purchases Ledger** - Records all transactions related to purchases that your business entity makes or between you and your suppliers. Showcases the amount you pay your suppliers or the amount your to be paid for the purchases.The following details are recorded: _type and quantity of goods purchased_, _list of suppliers_, _purchases involving huge sums of money_.
  - **General Ledger** - Contains all the ledger accounts other than sales and purchases accounts. Various sub ledgers* can be provided with requisite details to prepare a single General Ledger.The General Ledger accounts provide information that helps prepare financial statements. The General Ledger Accounts include: \_Assets*, _Liabilities_, _Shareholder's Equity_, _Revenue_, and _Expenses_.

- **Account** - Contain all _Debit_ and _Credit_ transactions affecting them. Also include detailed information about each _transaction_, such as date, description, amount, and may include descriptive information on what the transaction was. At least two accounts are managed whenever a transaction takes place
- **Transaction** - They are `closed out` or `summarized` to the general ledger. Each transaction affects at least two accounts, and each entry has at least one debit and one credit transaction
- **Trial Balance** - Validates the ledgers accuracy. A report that lists every general ledger account and its balance, makes adjustments easier to check and errors easier to locate.
- **General Ledger Reconciliation** - helps ensure accuracy of the information contained win your Accounts

- **Financial Statement** -

### General Journal

- **General Journal** - records in order they occur
- **Entry**
- **Transaction**
