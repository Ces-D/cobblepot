use chrono::{DateTime, TimeDelta, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::account::AccountBalance;
use crate::code::{AccountType, VagueAccountType};

use crate::currency::{Amount, ExchangeRate};
use crate::summary::{BalanceSummarizer, Summary};

/// Reveals an entities's revenue, expenses, gains, and losses during a particular period
pub struct ProfitLossStatement {
    period_start: DateTime<Utc>,
    time_delta: TimeDelta,
    sub_periods: u8,
    statements: Vec<BalanceSummarizer>,
}

impl ProfitLossStatement {
    pub fn new(
        period_start: DateTime<Utc>,
        time_delta: TimeDelta,
        sub_periods: u8,
    ) -> ProfitLossStatement {
        let mut statements: Vec<BalanceSummarizer> = Vec::new();

        assert!(
            period_start + time_delta > chrono::offset::Utc::now(),
            "Report period end date must occur before current date"
        );

        for _ in 0..sub_periods {
            statements.push(BalanceSummarizer::default())
        }

        ProfitLossStatement { statements, time_delta, period_start, sub_periods }
    }

    fn _retrieve_statement_index(&self, target: DateTime<Utc>) -> usize {
        let target_delta = target - self.period_start;
        let single_period_delta = self.time_delta / self.sub_periods.into();
        let index = target_delta.num_seconds() / single_period_delta.num_seconds();
        index.to_usize().unwrap()
    }

    pub fn include_balance(&mut self, balance: AccountBalance, account_type: AccountType) {
        assert!(
            balance.timestamp < self.period_start + self.time_delta,
            "Account balance timestamp must not occur after the Statements end period",
        );
        assert!(
            balance.timestamp >= self.period_start,
            "Account balance timestamp must not occur before the Statements start period"
        );

        let insert_index = self._retrieve_statement_index(balance.timestamp);
        match self.statements.get_mut::<usize>(insert_index) {
            Some(summarizer) => {
                summarizer.include(balance, account_type);
            },
            None => panic!("Unable to access summarizer at index {}", insert_index),
        }
    }

    pub fn total_income(&self, periods: u8) -> Summary {
        assert!(periods <= self.sub_periods, "Target periods must be <= total periods");
        let mut current_total = Summary {
            debits: Amount::new(ExchangeRate::default()),
            credits: Amount::new(ExchangeRate::default()),
        };
        if periods == self.sub_periods {
            for sub_summary in self.statements.iter() {
                let income_summary =
                    sub_summary.summarize_vague_account_type(VagueAccountType::Income);
                current_total.debits = income_summary.debits;
                current_total.credits = income_summary.credits;
            }
        } else {
            let sub_statements = self.statements.iter().take(periods.to_usize().unwrap());
            for sub_summary in sub_statements {
                let income_summary =
                    sub_summary.summarize_vague_account_type(VagueAccountType::Income);
                current_total.debits = income_summary.debits;
                current_total.credits = income_summary.credits;
            }
        }

        current_total
    }

    pub fn total_expenses(&self, periods: u8) -> Summary {
        assert!(periods <= self.sub_periods, "Target periods must be <= total periods");
        let mut current_total = Summary {
            debits: Amount::new(ExchangeRate::default()),
            credits: Amount::new(ExchangeRate::default()),
        };
        if periods == self.sub_periods {
            for sub_summary in self.statements.iter() {
                let expense_summary =
                    sub_summary.summarize_vague_account_type(VagueAccountType::Expense);
                current_total.debits = expense_summary.debits;
                current_total.credits = expense_summary.credits;
            }
        } else {
            let sub_statements = self.statements.iter().take(periods.to_usize().unwrap());
            for sub_summary in sub_statements {
                let expense_summary =
                    sub_summary.summarize_vague_account_type(VagueAccountType::Expense);
                current_total.debits = expense_summary.debits;
                current_total.credits = expense_summary.credits;
            }
        }

        current_total
    }

    pub fn gross_profit(&self, periods: u8) -> Summary {
        let total_income = self.total_income(periods);
        let total_expenses = self.total_expenses(periods);
        Summary {
            debits: total_income.debits - total_expenses.debits,
            credits: total_income.credits - total_expenses.credits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
