use crate::chart_of_accounts::code::AccountCode;
use crate::chart_of_accounts::enums;

pub struct Account {
    //  The name or title of the account, which describes the nature and purpose of the account.
    account_name: String,
    // A brief description or explanation of the account and its purpose, which provides additional context for the account.
    account_description: Option<String>,

    /// A chart of accounts typically includes five main types of accounts: assets, liabilities, equity, revenues, and expenses.
    account_type: enums::AccountType,

    /// Accounts within each category are typically organized in a hierarchical structure, with sub-accounts nested under main accounts.
    sub_account_codes: Vec<AccountCode>,
    sub_accounts: Option<Vec<Account>>,

    // The balance of an account at the beginning of a specific accounting period.
    opening_balance: Option<String>,
    // The balance of the account at the end of the accounting period, which is calculated by adding the opening balance to the net balance for the period.
    closing_balance: Option<String>,
}
