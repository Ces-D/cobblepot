use std::iter::repeat_with;

use chrono::{Days, Months, NaiveDateTime, Utc};
use diesel::{Connection, QueryDsl, RunQueryDsl, dsl::insert_into};

use crate::{
    account::model::{InsertableAccount, JSONOpenAccount},
    balance::model::{InsertableBalance, JSONOpenBalance},
    infrastructure::database::PoolConnection,
    schema::{account::dsl as account_dsl, balance::dsl as balance_dsl},
    shared::AccountType,
};
use cobblepot_core::error::CobblepotResult;

pub fn create_dummy_open_balance(account_id: i32) -> JSONOpenBalance {
    JSONOpenBalance {
        memo: Some(repeat_with(fastrand::alphanumeric).take(10).collect()),
        amount: 100.0,
        entered_on: None,
        account_id,
    }
}

pub fn create_dummy_open_account(account_type: Option<AccountType>) -> JSONOpenAccount {
    let account_types = vec![AccountType::Asset, AccountType::Liability];
    JSONOpenAccount {
        name: repeat_with(fastrand::alphanumeric).take(10).collect(),
        description: None,
        owner: None,
        account_type: Some(
            account_type
                .unwrap_or_else(|| account_types[fastrand::usize(0..account_types.len() - 1)]),
        ),
        opened_on: None,
        closed_on: None,
    }
}

pub fn create_dummy_open_accounts(account_types: Option<AccountType>) -> Vec<JSONOpenAccount> {
    let mut accounts: Vec<JSONOpenAccount> = vec![];

    for i in 0..13 {
        let mut acc = create_dummy_open_account(account_types);
        // Every account is at least 12 months old
        acc.opened_on = Utc::now().checked_sub_months(Months::new(i + 12));

        accounts.push(acc);
    }

    accounts
}

pub fn create_dummy_account_balances(
    account_id: i32,
    entered_on: chrono::DateTime<Utc>,
    is_monthly: bool,
) -> Vec<JSONOpenBalance> {
    let mut balances = vec![];

    for i in 0..10 {
        let mut balance = create_dummy_open_balance(account_id);
        match is_monthly {
            true => balance.entered_on = entered_on.checked_add_months(Months::new(i)),
            false => balance.entered_on = entered_on.checked_add_days(Days::new(i as u64)),
        }
        let random = fastrand::u32(1..=10) * i * 100;
        balance.amount = random as f32;

        balances.push(balance);
    }

    balances
}

/// seeds the database creating a years worth of accounts. with 10 months worth of balances per account
pub fn seed_database(
    mut connection: PoolConnection,
    account_types: Option<AccountType>,
) -> CobblepotResult<()> {
    connection.transaction(|conn| {
        let insertable_accounts: Vec<InsertableAccount> =
            create_dummy_open_accounts(account_types).into_iter().map(|v| v.into()).collect();
        insert_into(account_dsl::account).values(insertable_accounts).execute(conn)?;
        let inserted_data = account_dsl::account
            .select((account_dsl::id, account_dsl::opened_on))
            .load::<(i32, NaiveDateTime)>(conn)?;
        let insertable_balances: Vec<Vec<InsertableBalance>> = inserted_data
            .into_iter()
            .map(|v| {
                let (account_id, opened_on) = v;
                let balances = create_dummy_account_balances(account_id, opened_on.and_utc(), true);
                let insertable: Vec<InsertableBalance> =
                    balances.into_iter().map(|b| b.into()).collect();
                return insertable;
            })
            .collect();

        for insertable in insertable_balances.into_iter() {
            insert_into(balance_dsl::balance).values(insertable).execute(conn)?;
        }
        Ok(())
    })
}
