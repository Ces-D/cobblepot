/// A chart of accounts is a listing of all the accounts used in an accounting system to classify and record financial transactions.
/// It provides a standardized framework for organizing financial information that makes it easier to produce financial statements and reports.
struct ChartOfAccounts {
    accounts: Vec<Account>,
    accounting_period: AccountingPeriod,
}

struct Account {
    //  The name or title of the account, which describes the nature and purpose of the account.
    account_name: String,
    // A brief description or explanation of the account and its purpose, which provides additional context for the account.
    account_description: Option<String>,

    /// A chart of accounts typically includes five main types of accounts: assets, liabilities, equity, revenues, and expenses.
    account_type: AccountType,

    /// Each account is assigned a unique account number or code that helps to identify and categorize the account.
    account_code: AccountCode,
    /// Accounts within each category are typically organized in a hierarchical structure, with sub-accounts nested under main accounts.
    sub_account_codes: Vec<AccountCode>,
    sub_accounts: Option<Vec<Account>>,

    // The balance of an account at the beginning of a specific accounting period.
    opening_balance: Option<Money>,
    // The balance of the account at the end of the accounting period, which is calculated by adding the opening balance to the net balance for the period.
    closing_balance: Option<Money>,
}

enum AccountingPeriod {
    Yearly,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
}

/// The numbering system used in a chart of accounts should be logical and consistent, making it easy to add new accounts as needed and to generate reports and financial statements.
struct AccountCode {
    // The first digit of an account number indicates the account type.
    account_type: AccountType,
    // The following digits indicate the sub-account types.
    sub_account_types: Vec<AccountType>,
    // The last digit is used to identify the specific index id of this account in its
    // account-subaccounts section.
    index: i32,
}

// The classification of the account as an asset, liability, equity, revenue, or expense account, which determines how the account is treated for financial reporting purposes
enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

// TODO: reorganize these elements into various files
// TODO: read below and implement as struct methods
// A chart of accounts is a fundamental tool used in accounting to organize and categorize financial transactions. Here are some actions that can be taken on a chart of accounts:
//
// Adding Accounts: New accounts can be added to the chart of accounts to reflect changes in the business or organization's operations or to provide more detailed information about specific transactions.
//
// Deleting Accounts: Accounts that are no longer relevant or necessary can be deleted from the chart of accounts to simplify the accounting process and improve the accuracy of financial reporting.
//
// Modifying Accounts: Accounts can be modified to reflect changes in the business or organization's operations or to correct errors in the account structure or information.
//
// Reorganizing Accounts: Accounts can be reorganized or rearranged within the chart of accounts to improve the clarity and accuracy of financial reporting or to align with changes in the business or organization's operations.
//
// Consolidating Accounts: Accounts that have similar characteristics or functions can be consolidated into a single account to simplify the accounting process and improve the accuracy of financial reporting.
//
// Reviewing Accounts: Regular reviews of the chart of accounts are necessary to ensure that it remains up-to-date and reflects changes in the business or organization's financial reporting requirements.
//
// Customizing Accounts: A chart of accounts can be customized to suit the specific needs of a business or organization, while still following generally accepted accounting principles (GAAP).
//
// By taking these actions, businesses and organizations can ensure that their chart of accounts remains accurate, relevant, and useful for financial reporting and decision-making purposes.
