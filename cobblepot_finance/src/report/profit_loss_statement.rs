use chrono::{DateTime, TimeDelta, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::account::AccountBalance;
use crate::code::AccountType;
use crate::currency::Amount;

use super::summarizer::{BalanceSummarizer, Summary};

#[derive(Clone, Copy)]
pub enum ReportInterval {
    Daily,
    Weekly,
    Monthly,
    Annually,
}

/// Reveals an entities's revenue, expenses, gains, and losses during a particular period
pub struct ProfitLossStatement {
    period_start: DateTime<Utc>,
    intervals: u8,
    report_interval: ReportInterval,
    statements: Vec<BalanceSummarizer>,
}

impl ProfitLossStatement {
    pub fn new(
        period_start: DateTime<Utc>,
        report_interval: ReportInterval,
        intervals: u8,
    ) -> ProfitLossStatement {
        let mut statements: Vec<BalanceSummarizer> = Vec::new();

        let period_end = ProfitLossStatement::_period_end(period_start, report_interval, intervals);

        assert!(
            period_end > chrono::offset::Utc::now(),
            "Report period end date must occur before current date"
        );

        for _ in 0..intervals {
            statements.push(BalanceSummarizer::default())
        }

        ProfitLossStatement { statements, report_interval, intervals, period_start }
    }

    fn _delta(report_interval: ReportInterval, intervals: i64) -> TimeDelta {
        match report_interval {
            ReportInterval::Daily => TimeDelta::days(intervals),
            ReportInterval::Weekly => TimeDelta::weeks(intervals),
            ReportInterval::Monthly => TimeDelta::weeks(4 * intervals),
            ReportInterval::Annually => TimeDelta::weeks(52 * intervals),
        }
    }

    fn _period_end(
        period_start: DateTime<Utc>,
        report_interval: ReportInterval,
        intervals: u8,
    ) -> DateTime<Utc> {
        period_start + ProfitLossStatement::_delta(report_interval, intervals.into())
    }

    //TODO: Could implement a binary search since the statements Vec should always be in order
    fn _retrieve_statement_index(&self, timestamp: DateTime<Utc>) -> u8 {
        let mut sub_period_end =
            ProfitLossStatement::_period_end(self.period_start, self.report_interval, 1);
        let mut statement_index: u8 = 0;

        while sub_period_end < timestamp {
            statement_index += 1;
            sub_period_end =
                ProfitLossStatement::_period_end(sub_period_end, self.report_interval, 1);
        }

        statement_index
    }

    pub fn include_balance(&mut self, balance: AccountBalance, account_type: AccountType) {
        assert!(
            balance.timestamp
                < ProfitLossStatement::_period_end(
                    self.period_start,
                    self.report_interval,
                    self.intervals,
                ),
            "Account balance timestamp must not occur after the Statements end period",
        );
        assert!(
            balance.timestamp >= self.period_start,
            "Account balance timestamp must not occur before the Statements start period"
        );

        let insert_index = self._retrieve_statement_index(balance.timestamp);
        let balance_summarizer = self.statements.get_mut(insert_index.to_usize().unwrap()).unwrap();
        balance_summarizer.include_by_balance(balance, account_type);
    }

    pub fn total_income(&self) -> Summary {
        let mut current_total = Summary::default();
        for sub_summary in self.statements.iter() {
            current_total += sub_summary.summarize_income();
        }
        current_total
    }

    pub fn total_expenses(&self) -> Summary {
        let mut current_total = Summary::default();
        for sub_summary in self.statements.iter() {
            current_total += sub_summary.summarize_expense();
        }
        current_total
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

// TODO: write tests for this struct
