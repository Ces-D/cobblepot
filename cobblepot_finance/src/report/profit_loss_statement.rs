use crate::account::AccountBalance;
use crate::code::AccountType;
use crate::currency::Amount;

use super::summarizer::{BalanceSummarizer, Summary};

/// Reveals an entities's revenue, expenses, gains, and losses during a particular period
pub struct ProfitLossStatement {
    summarizer: BalanceSummarizer,
    start_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    end_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl ProfitLossStatement {
    pub fn new() -> ProfitLossStatement {
        ProfitLossStatement {
            summarizer: BalanceSummarizer::default(),
            start_timestamp: None,
            end_timestamp: None,
        }
    }

    pub fn include_balance(&mut self, balance: AccountBalance, account_type: AccountType) {
        match self.start_timestamp {
            Some(current_start) => {
                if current_start > balance.timestamp {
                    self.start_timestamp = Some(balance.timestamp);
                }
            },
            None => self.start_timestamp = Some(balance.timestamp),
        }

        match self.end_timestamp {
            Some(current_end) => {
                if current_end < balance.timestamp {
                    self.end_timestamp = Some(balance.timestamp);
                }
            },
            None => self.start_timestamp = Some(balance.timestamp),
        }
        self.summarizer.include_by_balance(balance, account_type)
    }

    pub fn total_income(&self) -> Summary {
        self.summarizer.summarize_income()
    }

    pub fn total_expenses(&self) -> Summary {
        self.summarizer.summarize_expense()
    }

    pub fn gains(&self) -> Amount {
        todo!()
    }

    pub fn losses(&self) -> Amount {
        todo!()
    }

    pub fn net_income(&self) -> Amount {
        todo!()
    }

    pub fn earnings_per_share(&self) -> Amount {
        todo!()
    }
}
