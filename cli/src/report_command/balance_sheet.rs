use chrono::Local;
use num_enum::FromPrimitive;

use crate::{
    inform,
    logger::table::{ColumnConfig, Table},
    report_command::dto::LatestBalanceRow,
    shared::{AccountType, format_money_usd},
};

#[derive(Debug)]
struct Entry {
    ids: (i32, i32),
    name: String,
    balance: f32,
    entered_on: chrono::NaiveDateTime,
}

/// [BalanceSheet Definition](https://www.netsuite.com/portal/resource/articles/accounting/balance-sheet.shtml)
#[derive(Default, Debug)]
pub struct BalanceSheet {
    start: Option<chrono::NaiveDateTime>,
    end: Option<chrono::NaiveDateTime>,
    assets: Vec<Entry>,
    liabilities: Vec<Entry>,
}
impl BalanceSheet {
    fn update_report_date_range(&mut self, entered_on: chrono::NaiveDateTime) {
        if self.start.is_none() || Some(entered_on) < self.start {
            self.start = Some(entered_on);
        }
        if self.end.is_none() || Some(entered_on) > self.end {
            self.end = Some(entered_on);
        }
    }
    pub fn push_balance(&mut self, row: LatestBalanceRow) {
        self.update_report_date_range(row.entered_on.inner());
        let account_type = AccountType::from_primitive(row.account_type);
        match account_type {
            AccountType::Asset => {
                self.assets.push(Entry {
                    ids: (row.account_id, row.balance_id),
                    name: row.account_name,
                    balance: row.amount,
                    entered_on: row.entered_on.inner(),
                });
            }
            AccountType::Liability => {
                self.liabilities.push(Entry {
                    ids: (row.account_id, row.balance_id),
                    name: row.account_name,
                    balance: row.amount,
                    entered_on: row.entered_on.inner(),
                });
            }
        }
    }
    fn total_assets_value(&self) -> f32 {
        let mut total_value: f32 = 0.0;
        for asset in &self.assets {
            total_value += asset.balance;
        }
        total_value
    }
    fn total_liabilities_value(&self) -> f32 {
        let mut total_value: f32 = 0.0;
        for liability in &self.liabilities {
            total_value += liability.balance;
        }
        total_value
    }
    fn total_equity(&self) -> f32 {
        self.total_assets_value() - self.total_liabilities_value()
    }
    fn equity_to_assets_ratio(&self) -> f32 {
        self.total_equity() / self.total_assets_value()
    }
    fn debt_to_equity_ratio(&self) -> f32 {
        self.total_liabilities_value() / self.total_equity()
    }
    fn debt_to_assets_ratio(&self) -> f32 {
        self.total_liabilities_value() / self.total_assets_value()
    }
    pub fn display(&self) {
        inform!(
            "Balance Sheet:",
            format!(
                "{} - {}",
                self.start
                    .unwrap_or_else(|| Local::now().naive_utc())
                    .format("%Y-%m-%d")
                    .to_string(),
                self.end.unwrap_or_else(|| Local::now().naive_utc()).format("%Y-%m-%d").to_string()
            )
        );
        inform!("Total Assets:", format_money_usd(self.total_assets_value()));
        inform!("Total Liabilities:", format_money_usd(self.total_liabilities_value()));
        inform!("Total Equity:", format_money_usd(self.total_equity()));
        inform!("Debt to Assets Ratio:", self.debt_to_assets_ratio());
        inform!("Equity to Assets Ratio:", self.equity_to_assets_ratio());
        inform!("Debt to Equity Ratio:", self.debt_to_equity_ratio());
        println!("\n");
        let mut assets_table = Table::new(vec![
            ColumnConfig::new("Asset Name"),
            ColumnConfig::new("Balance").min_width(20),
            ColumnConfig::new("Entered On"),
        ]);
        for asset in &self.assets {
            assets_table.push_row(vec![
                &asset.name,
                &format_money_usd(asset.balance),
                &asset.entered_on.format("%Y-%m-%d").to_string(),
            ]);
        }
        assets_table.display();
        println!("\n");
        let mut liabilities_table = Table::new(vec![
            ColumnConfig::new("Liability Name"),
            ColumnConfig::new("Balance").min_width(20),
            ColumnConfig::new("Entered On"),
        ]);
        for liability in &self.liabilities {
            liabilities_table.push_row(vec![
                &liability.name,
                &format_money_usd(liability.balance),
                &liability.entered_on.format("%Y-%m-%d").to_string(),
            ]);
        }
        liabilities_table.display();
    }
}
