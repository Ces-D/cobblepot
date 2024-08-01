use crate::code::{AccountType, VagueAccountType};
use crate::currency::{Amount, ExchangeRate};
use crate::historical_record::Summary;

pub struct BalanceSummarizer {
    summaries: std::collections::HashMap<AccountType, Summary>,
}

impl Default for BalanceSummarizer {
    fn default() -> Self {
        BalanceSummarizer { summaries: std::collections::HashMap::new() }
    }
}

impl BalanceSummarizer {
    pub fn include(&mut self, balance: Summary, account_type: AccountType) {
        match self.summaries.get_mut(&account_type) {
            Some(prev_summary) => {
                prev_summary.credits = prev_summary.credits + balance.credits;
                prev_summary.debits = prev_summary.debits + balance.debits;
            },
            None => {
                self.summaries.insert(
                    account_type,
                    Summary { debits: balance.debits, credits: balance.credits },
                );
            },
        }
    }

    pub fn summarize_vague_account_type(&self, target_type: VagueAccountType) -> Summary {
        let mut total_debits = Amount::new(ExchangeRate::default());
        let mut total_credits = Amount::new(ExchangeRate::default());
        let target_accounts: Vec<&AccountType> =
            self.summaries.keys().filter(|x| VagueAccountType::from(*x) == target_type).collect();

        for key in target_accounts {
            let summary = self.summaries.get(key).expect("Value for key should exist");
            total_debits = total_debits + summary.debits;
            total_credits = total_credits + summary.credits;
        }

        Summary { debits: total_debits, credits: total_credits }
    }

    pub fn summarize_account_type(&self, target_type: &AccountType) -> Summary {
        let summary = self.summaries.get(target_type).expect("Value for key should exist");
        summary.clone()
    }
}
