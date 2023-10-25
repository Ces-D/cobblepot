use std::collections::HashMap;
use std::ops::Add;

use self::account::Account;
use self::account_code::AccountCode;
use self::account_type::AccountType;

pub mod account;
pub mod account_code;
pub mod account_type;

/// AKA: Chart of Accounts
/// **Chart of Accounts** - Is a listing of all the accounts used in an accounting system to classify and record financial transactions. It provides a standardized framework for organizing financial information that makes it easier to produce financial statements and reports.
/// The account manager is responsible for creating and getting accounts and account codes
struct AccountManager {
    chart_of_accounts: HashMap<AccountCode, Account>,
}

impl AccountManager {
    pub fn new() -> AccountManager {
        AccountManager { chart_of_accounts: HashMap::new() }
    }

    ///  Creates a new account, checking if a similar account already exists
    pub fn open_account(
        &mut self,
        name: String,
        description: String,
        account_type: AccountType,
    ) -> Option<Account> {
        if self.similar_account_exists(name.clone(), account_type.clone()) {
            panic!("Similar account already exists");
        } else {
            let account_code =
                AccountCode::new(account_type.clone(), self.next_code_index(account_type));
            let account = Account::new(name, description, account_code.clone());
            // an account should not be able to be opened twice, since it has a unique index
            self.chart_of_accounts.insert(account_code, account)
        }
    }

    pub fn get_account(&self, account_code: &AccountCode) -> Option<&Account> {
        self.chart_of_accounts.get(account_code)
    }

    /// Checks if a similar account exists by name and account type
    fn similar_account_exists(&self, name: String, account_type: AccountType) -> bool {
        self.chart_of_accounts.values().any(|account| {
            account.name.to_lowercase() == name.to_lowercase()
                && account.get_code().extract_account_type() == account_type
        })
    }

    /// Gets the next index for a given account type
    fn next_code_index(&self, account_type: AccountType) -> u32 {
        let matched_codes = self
            .chart_of_accounts
            .keys()
            .filter(|code| code.extract_account_type() == account_type)
            .collect::<Vec<&AccountCode>>()
            .len();

        u32::try_from(matched_codes).unwrap().add(1)
    }
}
