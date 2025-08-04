use actix_web::{Scope, web};
use chrono::{DateTime, Months, Utc};
use diesel::{
    Connection, ExpressionMethods, RunQueryDsl,
    dsl::insert_into,
    query_dsl::methods::{FilterDsl, OrderDsl},
};

use crate::{
    account::model::Account,
    apply::model::{
        AppliedFinancialMarketInstruments, AppliedRecurringTransaction, JSONApplyMarketInstrument,
        JSONApplyRecurringTransaction,
    },
    balance::model::{Balance, InsertableBalance, JSONOpenBalance},
    financial_market::{model::MarketInstrument, service::get_current_market_value},
    infrastructure::database::{DbPool, PoolConnection},
    recurring_transaction::{model::RecurringTransaction, recurrance::recurrance_dates},
    schema::{
        account::dsl::{account, id as acct_id},
        balance::dsl::{account_id, balance, entered_on},
        market_instrument::dsl::{account_id as market_account_id, market_instrument},
        recurring_transactions::dsl::{id as transaction_id, recurring_transactions},
    },
    shared::AccountType,
};
use cobblepot_core::error::{CobblepotError, CobblepotResult};

fn apply_recurring_balance(
    mut connection: PoolConnection,
    accnt_id: i32,
    recurring_account_type: AccountType,
    total_applied_amount: f32,
    recurring_name: String,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    recurring_id: i32,
) -> CobblepotResult<Balance> {
    connection.transaction(|conn| {
        let previous_balance =
            OrderDsl::order(FilterDsl::filter(balance, account_id.eq(accnt_id)), entered_on.desc())
                .first::<Balance>(conn)?;
        let new_balance_amount = match AccountType::from(recurring_account_type) {
            AccountType::Asset => previous_balance.amount + total_applied_amount,
            AccountType::Liability => previous_balance.amount - total_applied_amount,
        };
        let open_balance = JSONOpenBalance {
            memo: Some(format!(
                "Applied recurring transaction. Name: {} From: {} To: {} Id: {}",
                recurring_name,
                from.to_rfc2822(),
                to.to_rfc2822(),
                recurring_id
            )),
            amount: new_balance_amount,
            account_id: accnt_id,
            entered_on: None,
        };
        let insertable: InsertableBalance = open_balance.into();
        let res = insert_into(balance).values(insertable).get_result::<Balance>(conn)?;
        Ok(res)
    })
}

async fn insert_applied_recurring_transaction(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONApplyRecurringTransaction>,
) -> CobblepotResult<AppliedRecurringTransaction> {
    let args = payload.into_inner();
    let from = args.from.unwrap_or(
        Utc::now().checked_sub_months(Months::new(3)).expect("Failed to calculate date"),
    );
    let to = args.to.unwrap_or(Utc::now());

    if from >= to {
        return Err(CobblepotError::LogicError("from date must be before to date".to_string()));
    }

    let first_pool = pool.clone();
    let transaction: Result<RecurringTransaction, CobblepotError> = web::block(move || {
        let mut conn = first_pool.get().unwrap();
        let res = FilterDsl::filter(recurring_transactions, transaction_id.eq(args.id))
            .first::<RecurringTransaction>(&mut conn)?;
        Ok(res)
    })
    .await?;

    match transaction {
        Ok(recurring) => {
            let dates = recurrance_dates(recurring.rrule, recurring.start_date)?;
            let mut filtered = dates.iter().filter(|x| from >= **x && to <= **x);
            let mut applied_count = 0;
            let mut total_applied_amount: f32 = 0.0;
            let applied_on: Vec<DateTime<Utc>> = vec![];
            while filtered.next().is_some() {
                applied_count += 1;
                total_applied_amount += recurring.amount;
            }

            let cloned_name = recurring.name.clone();
            let new_balance = web::block(move || {
                let conn = pool.get().unwrap();
                apply_recurring_balance(
                    conn,
                    recurring.account_id,
                    recurring.account_type.into(),
                    total_applied_amount,
                    cloned_name,
                    from,
                    to,
                    recurring.id,
                )
            })
            .await?;

            match new_balance {
                Ok(b) => Ok(AppliedRecurringTransaction {
                    id: recurring.id,
                    account_id: recurring.account_id,
                    name: recurring.name,
                    description: recurring.description,
                    account_type: recurring.account_type,

                    update_balance_id: b.id,
                    applied_from: from,
                    applied_to: to,
                    applied_count,
                    amount: recurring.amount,
                    total_applied_amount,
                    applied_on,
                }),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

async fn insert_applied_financial_market_instruments(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONApplyMarketInstrument>,
) -> CobblepotResult<AppliedFinancialMarketInstruments> {
    let mut conn = pool.get().unwrap();
    let accnt_id = payload.id;

    let instruments = FilterDsl::filter(market_instrument, market_account_id.eq(accnt_id))
        .load::<MarketInstrument>(&mut conn)?;

    let mut total_market_value = 0.0;

    for instrument in &instruments {
        total_market_value +=
            get_current_market_value(instrument.ticker.clone(), instrument.quantity).await?;
    }

    let acc = FilterDsl::filter(account, acct_id.eq(accnt_id)).first::<Account>(&mut conn)?;

    let open_balance = JSONOpenBalance {
        memo: Some(format!("Applied market instruments value for account {}", acc.name)),
        amount: total_market_value as f32,
        account_id: accnt_id,
        entered_on: None,
    };
    let insertable: InsertableBalance = open_balance.into();
    let new_balance = insert_into(balance).values(insertable).get_result::<Balance>(&mut conn)?;

    Ok(AppliedFinancialMarketInstruments {
        applied_count: instruments.len(),
        new_balance_id: new_balance.id,
    })
}

pub fn apply_scope() -> Scope {
    web::scope("/apply")
        .route("/recurring_transaction", web::post().to(insert_applied_recurring_transaction))
        .route("/market_instruments", web::post().to(insert_applied_financial_market_instruments))
}
