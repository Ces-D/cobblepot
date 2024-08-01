use crate::account::Account;
use crate::code::{AccountType, VagueAccountType};

use crate::historical_record::Summary;
use crate::summarizer::BalanceSummarizer;

pub struct BalanceSheet {
    summarizer: BalanceSummarizer,
}

impl BalanceSheet {
    pub fn new(accounts: Vec<Account>) -> BalanceSheet {
        let mut summarizer = BalanceSummarizer::default();
        for account in accounts {
            summarizer.include(account.balance.summary, account.code.account_type)
        }

        BalanceSheet { summarizer }
    }

    pub fn total_assets(&self) -> Summary {
        self.summarizer.summarize_vague_account_type(VagueAccountType::Asset)
    }

    pub fn total_liabilities(&self) -> Summary {
        self.summarizer.summarize_vague_account_type(VagueAccountType::Liability)
    }

    pub fn total_equity(&self) -> Summary {
        self.summarizer.summarize_vague_account_type(VagueAccountType::Equity)
    }

    pub fn total_of_account_type(&self, target_account_type: &AccountType) -> Summary {
        self.summarizer.summarize_account_type(target_account_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
