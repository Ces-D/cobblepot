use std::ops::{Add, AddAssign};

use rust_decimal::Decimal;

use crate::account::{Account, AccountBalance};
use crate::code::AccountType;

pub struct Summary {
    pub debits: Decimal,
    pub credits: Decimal,
}

impl Add for Summary {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Summary { debits: self.debits + rhs.debits, credits: self.credits + rhs.credits }
    }
}

impl AddAssign for Summary {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { debits: self.debits + rhs.debits, credits: self.credits + rhs.credits };
    }
}

impl Default for Summary {
    fn default() -> Self {
        Summary { debits: Decimal::ZERO, credits: Decimal::ZERO }
    }
}

pub struct BalanceSummarizer {
    summaries: std::collections::HashMap<AccountType, Summary>,
}

impl Default for BalanceSummarizer {
    fn default() -> Self {
        BalanceSummarizer { summaries: std::collections::HashMap::new() }
    }
}

impl BalanceSummarizer {
    pub fn include_by_account(&mut self, account: &Account) {
        let account_type = account.code().account_type;
        self.include_by_balance(account.balance, account_type)
    }

    pub fn include_by_balance(&mut self, balance: AccountBalance, account_type: AccountType) {
        match self.summaries.get_mut(&account_type) {
            Some(prev_summary) => {
                prev_summary.credits += balance.credit_balance.value_in_usd();
                prev_summary.debits += balance.debit_balance.value_in_usd();
            },
            None => {
                self.summaries.insert(
                    account_type,
                    Summary {
                        debits: balance.debit_balance.value_in_usd(),
                        credits: balance.credit_balance.value_in_usd(),
                    },
                );
            },
        }
    }

    fn summarize_by_type(&self, condition: fn(account_type: AccountType) -> bool) -> Summary {
        let mut total_debits = Decimal::ZERO;
        let mut total_credits = Decimal::ZERO;

        for key in self.summaries.keys() {
            if condition(key.clone()) {
                let summary = self.summaries.get(key).expect("Value for key should exist");
                total_debits += summary.debits;
                total_credits += summary.credits;
            }
        }

        Summary { debits: total_debits, credits: total_credits }
    }

    pub fn summarize_assets(&self) -> Summary {
        self.summarize_by_type(AccountType::is_asset)
    }
    pub fn summarize_liabilities(&self) -> Summary {
        self.summarize_by_type(AccountType::is_liability)
    }
    pub fn summarize_equity(&self) -> Summary {
        self.summarize_by_type(AccountType::is_equity)
    }
    pub fn summarize_income(&self) -> Summary {
        self.summarize_by_type(AccountType::is_income)
    }
    pub fn summarize_expense(&self) -> Summary {
        self.summarize_by_type(AccountType::is_expense)
    }
}
