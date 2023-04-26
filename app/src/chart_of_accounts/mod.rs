use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

mod account;
mod code;
mod enums;
mod gaap;

/// A chart of accounts is a listing of all the accounts used in an accounting system to classify and record financial transactions.
/// It provides a standardized framework for organizing financial information that makes it easier to produce financial statements and reports.
struct ChartOfAccounts {
    accounts: HashMap<code::AccountCode, account::Account>,
    accounting_period: enums::AccountingPeriod,
}

// A chart of accounts is a fundamental tool used in accounting to organize and categorize financial transactions. Here are some actions that can be taken on a chart of accounts:
impl ChartOfAccounts {
    /// Adding Accounts: New accounts can be added to the chart of accounts to reflect changes in the business or organization's operations or to provide more detailed information about specific transactions.
    fn add_account(
        &mut self,
        account_code: code::AccountCode,
        account: account::Account,
    ) -> &account::Account {
        self.accounts.entry(account_code).or_insert(account)
    }

    /// Deleting Accounts: Accounts that are no longer relevant or necessary can be deleted from the chart of accounts to simplify the accounting process and improve the accuracy of financial reporting.
    fn delete_account(&mut self, account_code: code::AccountCode) -> Option<account::Account> {
        self.accounts.remove(&account_code)
    }

    /// Modifying Accounts: Accounts can be modified to reflect changes in the business or organization's operations or to correct errors in the account structure or information.
    fn modify_account(&mut self, account_code: code::AccountCode, updates: account::Account) {
        match self.accounts.entry(account_code) {
            Occupied(mut entry) => {
                entry.insert(updates);
            },
            Vacant(entry) => {
                entry.insert(updates);
            },
        }
    }
}

// TODO: read below and implement as struct methods
//
//
//
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
//

//
// TODO: implement these notes on consolidating accounts
// Accounts in a chart of accounts can be consolidated by combining multiple accounts into a single account.
// Consolidation is typically done to simplify the chart of accounts and to provide a more comprehensive and meaningful view of financial data.

// To consolidate accounts, businesses can follow these steps:
//
// Identify the accounts that need to be consolidated. This may involve reviewing the chart of accounts and identifying accounts that have similar characteristics or functions.
//
// Create a new account in the chart of accounts to represent the consolidated accounts. This account should have a unique account number and a descriptive name that reflects the accounts being consolidated.
//
// Transfer the balances from the individual accounts to the new consolidated account. This can be done by journal entry, adjusting entry, or other accounting methods.
//
// Update the financial statements to reflect the consolidated account. This involves replacing the individual accounts with the new consolidated account on the financial statements.
//
//

// TODO: implement these notes on reorganizing accounts,
// NOTE: reorganizing is linked to both reviewing and consolidating accounts
// Here are some steps that can be taken to reorganize accounts in a chart of accounts:
//
// Identify the accounts that need to be reorganized. This may involve reviewing the current account structure and identifying accounts that need to be grouped or categorized differently.
//
// Determine the new account structure. This may involve grouping accounts by function, activity, or other criteria that better reflect the company's current business operations.
//
// Create new accounts or sub-accounts. Depending on the new account structure, it may be necessary to create new accounts or sub-accounts to properly categorize transactions.
//
// Transfer balances to the new accounts. Once the new accounts are created, balances from the old accounts can be transferred to the new accounts using journal entries or adjusting entries.
//
// Update the financial statements to reflect the new account structure. This involves replacing the old accounts with the new accounts on the financial statements.
//
// By reorganizing accounts in a chart of accounts, businesses can improve the accuracy and relevance of financial reporting and analysis, making it easier to identify trends, track financial performance, and make informed decisions.
