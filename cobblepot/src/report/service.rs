use std::collections::HashMap;

use crate::{
    infrastructure::database::{DbPool, PoolConnection},
    recurring_transaction::recurrance::{recurrance_dates, recurrance_status},
    report::model::{
        AccountBalance, AccountDeepDive, BalanceSheet, ChangeSnapShot, ChangeTimeline,
        JSONOpenReport, LoadAccountBalance, SimpleRecurringTransaction,
    },
    schema::{
        account::dsl as account_dsl, balance::dsl as balance_dsl,
        recurring_transactions::dsl as recurring_dsl,
    },
    shared::{AccountType, RecurringStatus},
};
use actix_web::{Scope, web};
use chrono::{DateTime, Datelike, Months, NaiveDate, Utc};
use cobblepot_core::error::{CobblepotError, CobblepotResult};

fn get_balance_sheet_data(
    mut connection: PoolConnection,
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
                    amount,
                    account_type: account_t.into(),
                    name,
                }
            })
            .collect();
        Ok(res)
    })
}

async fn create_balance_sheet_report(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenReport>,
) -> CobblepotResult<BalanceSheet> {
    let args = payload.into_inner();
    let from = args.from.unwrap_or(
        Utc::now().checked_sub_months(Months::new(3)).expect("Failed to calculate date"),
    );
    let to = args.to.unwrap_or(Utc::now());

    if from > to {
        return Err(CobblepotError::LogicError("from date must be before to date".to_string()));
    }

    web::block(move || {
        let conn = pool.get()?;
        let data = get_balance_sheet_data(conn, from, to)?;
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

        let assets_total =
            assets_latest.values().fold(0 as f32, |acc, balance| acc + balance.amount);
        let liabilities_total =
            liabilities_latest.values().fold(0 as f32, |acc, balance| acc + balance.amount);

        Ok(BalanceSheet {
            from,
            to,
            assets: assets_latest.values().map(AccountBalance::from).collect(),
            liabilities: liabilities_latest.values().map(AccountBalance::from).collect(),
            assets_total,
            liabilities_total,
            net_position: assets_total - liabilities_total,
        })
    })
    .await?
}

fn get_deep_dive_account_data(
    mut connection: PoolConnection,
    id: i32,
) -> CobblepotResult<(
    crate::account::model::Account,
    Vec<crate::balance::model::Balance>,
    Vec<crate::recurring_transaction::model::RecurringTransaction>,
)> {
    use crate::{
        account::model::Account, balance::model::Balance,
        recurring_transaction::model::RecurringTransaction,
    };
    use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
    connection.transaction(|conn| {
        let acct = account_dsl::account.filter(account_dsl::id.eq(id)).first::<Account>(conn)?;
        let balances = balance_dsl::balance
            .filter(balance_dsl::account_id.eq(id))
            .order(balance_dsl::entered_on.desc())
            .load::<Balance>(conn)?;
        // TODO: Change the status to be a calulated field. Add a new column called `was_closed` to signify manual closing prior to completion.
        // Currently, to update the status, we have to calculate them anyway, and then determine which ones need to be updated. Since we are still calculating in a loop. Lets just calculate them always.
        let transactions = recurring_dsl::recurring_transactions
            .filter(recurring_dsl::account_id.eq(id))
            .load::<RecurringTransaction>(conn)?;
        Ok((acct, balances, transactions))
    })
}

async fn create_deep_dive_account_report(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenReport>,
) -> CobblepotResult<AccountDeepDive> {
    let args = payload.into_inner();
    let from = args.from.unwrap_or(
        Utc::now().checked_sub_months(Months::new(6)).expect("Failed to calculate date"),
    );
    let to = args.to.unwrap_or(Utc::now());

    if from > to {
        return Err(CobblepotError::LogicError("From date must be before to date".to_string()));
    }

    if args.id.is_none() {
        return Err(CobblepotError::LogicError("Account ID is required".to_string()));
    }
    let id = args.id.unwrap();

    web::block(move || {
        let conn = pool.get()?;
        let (account, balances, recurrings) = get_deep_dive_account_data(conn, id)?;
        let mut recent: Option<AccountBalance> = None;
        let total_balance_entry_count = balances.len();

        if let Some(balance) = balances.first() {
            recent = Some(AccountBalance {
                account_id: account.id,
                balance_id: balance.id,
                name: account.name.clone(),
                entered_on: balance.entered_on.and_utc(),
                amount: balance.amount,
            });
        }

        // Determining like this because `balances` contains all entries since account creation
        let mut timeline_entry_count = 0; // total number of entries in timeframe
        let mut timeline_snaps: Vec<ChangeSnapShot> = Vec::new();
        let mut current_m: u32 = 0; // current loop's month
        let mut current_y: i32 = 0; // current loop's year
        let mut current_m_entry_count = 0; // current balance entries in month
        let mut current_m_total: f32 = 0.0; // sum of balance amounts in month

        for b in balances.into_iter() {
            if b.entered_on < to.naive_utc() && b.entered_on > from.naive_utc() {
                timeline_entry_count += 1;
                if b.entered_on.month() != current_m || b.entered_on.year() != current_y {
                    if current_m != 0 {
                        // We calculate the snapshot for this month since we have iterated through its balances
                        timeline_snaps.push(ChangeSnapShot {
                            timeframe: NaiveDate::from_ymd_opt(current_y, current_m, 1)
                                .expect("Invalid date. Possible error in loop tracking")
                                .and_hms_opt(0, 0, 0)
                                .expect("Invalid time")
                                .and_utc(),
                            average: current_m_total / current_m_entry_count as f32,
                        });
                    }

                    // starting new month track; resetting
                    current_m_entry_count = 0;
                    current_m = b.entered_on.month();
                    current_y = b.entered_on.year();
                    current_m_total = 0.0;
                }
                // adding balance to current month's total
                current_m_total += b.amount;
                current_m_entry_count += 1;
            }
        }

        let mut recurring: Vec<SimpleRecurringTransaction> = Vec::new();
        let mut total_recurring_monthly_asset = 0.0;
        let mut total_recurring_monthly_liability = 0.0;
        for r in recurrings.into_iter() {
            let simple = SimpleRecurringTransaction {
                id: r.id,
                name: r.name,
                amount: r.amount,
                account_type: r.account_type.into(),
                status: recurrance_status(r.rrule.clone(), r.start_date, r.closed)
                    .unwrap_or(RecurringStatus::Ongoing),
                apply_dates: recurrance_dates(r.rrule, r.start_date).unwrap_or_default(),
            };
            recurring.push(simple);
            match AccountType::from(r.account_type) {
                AccountType::Asset => total_recurring_monthly_asset += r.amount,
                AccountType::Liability => total_recurring_monthly_liability += r.amount,
            }
        }

        Ok(AccountDeepDive {
            id: account.id,
            name: account.name,
            description: account.description,
            owner: account.owner,
            account_type: account.account_type.into(),
            opened_on: account.opened_on.and_utc(),
            closed_on: account.closed_on.map(|v| v.and_utc()),

            total_entries: total_balance_entry_count,
            recent,

            timeline: ChangeTimeline {
                from,
                to,
                entry_count: timeline_entry_count,
                snapshots: timeline_snaps,
            },
            recurring,
            total_recurring_monthly_asset,
            total_recurring_monthly_liability,
        })
    })
    .await?
}

pub fn report_scope() -> Scope {
    web::scope("/report")
        .route("/balance_sheet", web::post().to(create_balance_sheet_report))
        .route("/account_deep_dive", web::post().to(create_deep_dive_account_report))
}

#[cfg(test)]
mod test {
    use crate::{report::model::JSONOpenReport, test_utils::seed_database};
    use actix_web::{App, test, web};
    use chrono::{Months, Utc};

    #[actix_web::test]
    async fn balance_sheet_report_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();

        let seed_conn = database_pool.get().unwrap();
        seed_database(seed_conn, None).unwrap();

        let app = test::init_service(
            App::new().app_data(web::Data::new(database_pool)).service(super::report_scope()),
        )
        .await;

        let balance_sheet_res = test::TestRequest::with_uri("/report/balance_sheet")
            .method(actix_web::http::Method::POST)
            .set_json(JSONOpenReport {
                report_type: crate::shared::ReportType::BalanceSheet,
                from: Utc::now().checked_sub_months(Months::new(12)),
                to: None,
                id: None,
            })
            .send_request(&app)
            .await;

        assert!(balance_sheet_res.status().is_success());
    }

    #[actix_web::test]
    async fn deep_dive_report_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();

        let seed_conn = database_pool.get().unwrap();
        seed_database(seed_conn, None).unwrap();

        let app = test::init_service(
            App::new().app_data(web::Data::new(database_pool)).service(super::report_scope()),
        )
        .await;

        let deep_dive_res = test::TestRequest::with_uri("/report/account_deep_dive")
            .method(actix_web::http::Method::POST)
            .set_json(JSONOpenReport {
                report_type: crate::shared::ReportType::BalanceSheet,
                from: Utc::now().checked_sub_months(Months::new(12)),
                to: None,
                id: Some(1),
            })
            .send_request(&app)
            .await;

        assert!(deep_dive_res.status().is_success());
    }
}
