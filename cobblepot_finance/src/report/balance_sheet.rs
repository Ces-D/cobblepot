use crate::account::Account;

use super::summarizer::{BalanceSummarizer, Summary};

pub struct BalanceSheet {
    summarizer: BalanceSummarizer,
}

impl BalanceSheet {
    pub fn new(accounts: Vec<Account>) -> BalanceSheet {
        let mut summarizer = BalanceSummarizer::default();
        for account in accounts {
            summarizer.include_by_account(&account)
        }

        BalanceSheet { summarizer }
    }

    pub fn total_assets(&self) -> Summary {
        self.summarizer.summarize_assets()
    }

    pub fn total_liabilities(&self) -> Summary {
        self.summarizer.summarize_liabilities()
    }

    pub fn total_equity(&self) -> Summary {
        self.summarizer.summarize_equity()
    }
}
