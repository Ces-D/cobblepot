use std::collections::HashMap;

use crate::{
    report::model::{AccountBalance, BalanceSheet, CliOpenReport, LoadAccountBalance},
    schema::{account::dsl as account_dsl, balance::dsl as balance_dsl},
    shared::{AccountType, CobblepotError, CobblepotResult},
};
use chrono::{DateTime, Months, Utc};
use diesel::SqliteConnection;

fn get_balance_sheet_data(
    mut connection: SqliteConnection,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> CobblepotResult<Vec<LoadAccountBalance>> {
    use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
    connection.transaction(|conn| {
        let data = balance_dsl::balance
            .filter(balance_dsl::entered_on.between(from.naive_utc(), to.naive_utc()))
            .order(balance_dsl::entered_on.desc())
            .inner_join(account_dsl::account)
            .select((
                balance_dsl::id,
                balance_dsl::account_id,
                balance_dsl::entered_on,
                balance_dsl::amount,
                account_dsl::account_type,
                account_dsl::name,
            ))
            .load::<(i32, i32, chrono::NaiveDateTime, f32, i32, String)>(conn)?;
        let res = data
            .into_iter()
            .map(|(balance_id, account_id, entered_on, amount, account_t, name)| {
                LoadAccountBalance {
                    balance_id,
                    account_id,
                    entered_on: entered_on.and_utc(),
                    amount: amount.into(),
                    account_type: account_t.into(),
                    name,
                }
            })
            .collect();
        Ok(res)
    })
}

pub fn create_balance_sheet_report(
    args: CliOpenReport,
    connection: SqliteConnection,
) -> CobblepotResult<BalanceSheet> {
    let from = args.from.unwrap_or(
        Utc::now().checked_sub_months(Months::new(3)).expect("Failed to calculate date"),
    );
    let to = args.to.unwrap_or(Utc::now());

    if from > to {
        return Err(CobblepotError::LogicError("from date must be before to date".to_string()));
    }

    let data = get_balance_sheet_data(connection, from, to)?;
    let mut assets_latest: HashMap<i32, LoadAccountBalance> = HashMap::new();
    let mut liabilities_latest: HashMap<i32, LoadAccountBalance> = HashMap::new();

    for b in data {
        if b.account_type == AccountType::Asset {
            assets_latest
                .entry(b.account_id)
                .and_modify(|existing| {
                    if b.entered_on > existing.entered_on {
                        *existing = b.clone()
                    }
                })
                .or_insert(b);
        } else if b.account_type == AccountType::Liability {
            liabilities_latest
                .entry(b.account_id)
                .and_modify(|existing| {
                    if b.entered_on > existing.entered_on {
                        *existing = b.clone()
                    }
                })
                .or_insert(b);
        }
    }

    let assets_total = assets_latest.values().fold(0 as f32, |acc, balance| acc + balance.amount);
    let liabilities_total =
        liabilities_latest.values().fold(0 as f32, |acc, balance| acc + balance.amount);

    Ok(BalanceSheet {
        from,
        to,
        assets: assets_latest.values().map(|v| AccountBalance::from(v)).collect(),
        liabilities: liabilities_latest.values().map(|v| AccountBalance::from(v)).collect(),
        assets_total,
        liabilities_total,
        net_position: assets_total - liabilities_total,
    })
}

pub fn create_deep_dive_account_report(
    args: CliOpenReport,
    mut connection: diesel::SqliteConnection,
) {
    // TODO: Implement deep dive account report creation
}
