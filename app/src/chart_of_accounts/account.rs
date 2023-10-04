use crate::chart_of_accounts::code::AccountCode;
use crate::chart_of_accounts::enums;
use rust_decimal::Decimal;
use rusty_money::{FormattableCurrency, Money};

pub struct Account {
    //  The name or title of the account, which describes the nature and purpose of the account.
    pub account_name: String,
    // A brief description or explanation of the account and its purpose, which provides additional context for the account.
    pub account_description: Option<String>,

    // Whether the account is closed or open. A closed account is one that is no longer in use, while an open account is one that is currently in use.
    pub is_closed: bool,

    /// A chart of accounts typically includes five main types of accounts: assets, liabilities, equity, revenues, and expenses.
    account_type: enums::AccountType,

    /// Each account is assigned a unique account number or code that helps to identify and categorize the account.
    account_code: AccountCode,

    /// Accounts within each category are typically organized in a hierarchical structure, with sub-accounts nested under main accounts.
    sub_account_codes: Vec<AccountCode>,
    sub_accounts: Option<Vec<Account>>,

    // The balance of an account at the beginning of a specific accounting period.
    opening_balance: Decimal,
    // The balance of the account at the end of the accounting period, which is calculated by adding the opening balance to the net balance for the period.
    closing_balance: Option<Decimal>,
}

impl Account {
    pub fn new<T: FormattableCurrency>(
        account_name: String,
        account_description: Option<String>,
        account_type: enums::AccountType,
        account_code: AccountCode,
        sub_account_codes: Vec<AccountCode>,
        sub_accounts: Option<Vec<Account>>,
        opening_balance: Money<T>,
    ) -> Account {
        Account {
            account_name,
            account_description,
            is_closed: false,
            account_type,
            account_code,
            sub_account_codes,
            sub_accounts,
            opening_balance: opening_balance.amount().clone(),
            closing_balance: None,
        }
    }

    pub fn get_account_type(&self) -> enums::AccountType {
        self.account_type.clone()
    }

    pub fn get_account_code(&self) -> AccountCode {
        self.account_code.clone()
    }

    pub fn get_sub_account_codes(&self) -> &Vec<AccountCode> {
        &self.sub_account_codes
    }

    pub fn get_opening_balance<'a, T: FormattableCurrency>(&'a self, currency: &'a T) -> Money<T> {
        Money::from_decimal(self.opening_balance.clone(), currency)
    }

    pub fn get_closing_balance<'a, T: FormattableCurrency>(
        &'a self,
        currency: &'a T,
    ) -> Option<Money<T>> {
        match self.closing_balance {
            Some(ref closing_balance) => {
                Some(Money::from_decimal(closing_balance.clone(), currency))
            },
            None => None,
        }
    }

    pub fn close_account(&mut self) {
        self.is_closed = true;
    }
}
