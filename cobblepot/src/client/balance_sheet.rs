use clap::Parser;

use super::shared::cli::{
    ISO8601_DATE_LONG_HELP, default_iso8601_variant_date, parse_iso8601_variant_date,
};

#[derive(Debug, Parser)]
#[command(about = "Edit an existing balance entry")]
pub struct BalanceSheetCommand {
    #[arg(short, help = "Date the balance sheet is summ", long_help=ISO8601_DATE_LONG_HELP, required = true, default_value_t = default_iso8601_variant_date(), value_parser = parse_iso8601_variant_date)]
    from: String,
    #[arg(short, help = "Date the balance sheet is summ", long_help=ISO8601_DATE_LONG_HELP, required = true, default_value_t = default_iso8601_variant_date(), value_parser = parse_iso8601_variant_date)]
    to: String,
}

pub mod query {
    use diesel::{Connection, QueryResult, RunQueryDsl};

    use crate::client::{
        account::AccountDetailed,
        balance::BalanceDetailed,
        shared::{
            cli::parse_iso8601_variant_datetime,
            report::{BalanceSheet, ReportItem},
            sql::AccountType,
        },
    };

    use super::BalanceSheetCommand;

    pub fn get_balances(
        params: BalanceSheetCommand,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<BalanceSheet> {
        connection.transaction(|conn| {
            let recent_act_balance_wthn_period = diesel::sql_query(
            "SELECT a.id, a.name, a.description, a.owner, a.account_type, a.opened_on, a.closed_on,
                    b.id, b.memo, b.amount, b.entered_on, b.account_id
             FROM account a
             JOIN balance b ON a.id = b.account_id
             JOIN (
                 SELECT account_id, MAX(entered_on) AS max_date
                 FROM balance
                 WHERE entered_on BETWEEN ? AND ?
                 GROUP BY account_id
             ) AS lb ON lb.account_id = b.account_id AND lb.max_date = b.entered_on"
        )            .bind::<diesel::sql_types::Text, _>(&params.from)
            .bind::<diesel::sql_types::Text, _>(&params.to)
            .load::<(BalanceDetailed, AccountDetailed)>(conn)?;

            let mut current_assets: Vec<ReportItem> = vec![];
            let mut current_liabilities: Vec<ReportItem> = vec![];
            let mut non_current_assets: Vec<ReportItem> = vec![];
            let mut non_current_liabilities: Vec<ReportItem> = vec![];

            for (balance_detailed, account_detailed) in recent_act_balance_wthn_period {
                let from_dt = parse_iso8601_variant_datetime(&params.from)
                    .expect("Unable to parse balance_detailed entered_on");
                let account_opened_on = parse_iso8601_variant_datetime(&account_detailed.opened_on)
                    .expect("Unable to parse account_detailed opened_on");

                if account_opened_on >= from_dt {
                    // is current
                    if account_detailed.account_type == AccountType::Asset {
                        current_assets.push(ReportItem {
                            name: account_detailed.name,
                            description: account_detailed.description,
                            owner: account_detailed.owner,
                            balance: balance_detailed.amount,
                        })
                    } else if account_detailed.account_type == AccountType::Liability {
                        current_liabilities.push(ReportItem {
                            name: account_detailed.name,
                            description: account_detailed.description,
                            owner: account_detailed.owner,
                            balance: balance_detailed.amount,
                        })
                    }
                } else {
                    // is non current
                    if account_detailed.account_type == AccountType::Asset {
                        non_current_assets.push(ReportItem {
                            name: account_detailed.name,
                            description: account_detailed.description,
                            owner: account_detailed.owner,
                            balance: balance_detailed.amount,
                        })
                    } else if account_detailed.account_type == AccountType::Liability {
                        non_current_liabilities.push(ReportItem {
                            name: account_detailed.name,
                            description: account_detailed.description,
                            owner: account_detailed.owner,
                            balance: balance_detailed.amount,
                        })
                    }
                }
            }

            Ok(BalanceSheet {
                from: params.from,
                to: params.to,
                current_assets,
                current_liabilities,
                non_current_assets,
                non_current_liabilities,
            })
        })
    }
}
