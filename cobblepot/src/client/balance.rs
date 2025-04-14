pub mod query {
    use diesel::{
        Connection, ExpressionMethods, JoinOnDsl, QueryDsl, QueryResult, RunQueryDsl, insert_into,
        query_dsl::methods::FilterDsl, update,
    };

    use crate::client::model::{BalanceDetailed, DisplayBalanceDetailed, EditBalance, NewBalance};

    pub fn insert_new_balance(
        new_balance: NewBalance,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<DisplayBalanceDetailed> {
        use crate::schema::account::dsl::{account, id as account_table_id, name as account_name};
        use crate::schema::balance::dsl::{
            amount, balance, entered_on, id as balance_table_id, memo,
        };
        connection.transaction(|conn| {
            let entered = insert_into(balance)
                .values(new_balance)
                .get_result::<BalanceDetailed>(conn)?;
            let balance_detailed = FilterDsl::filter(balance, balance_table_id.eq(entered.id))
                .inner_join(account.on(account_table_id.eq(entered.account_id)))
                .select((
                    balance_table_id,
                    memo,
                    amount,
                    entered_on,
                    account_name,
                    account_table_id,
                ))
                .first::<(i32, String, f32, String, String, i32)>(conn)?;
            Ok(DisplayBalanceDetailed {
                id: balance_detailed.0,
                memo: balance_detailed.1,
                amount: balance_detailed.2,
                entered_on: balance_detailed.3,
                account_name: balance_detailed.4,
                account_id: balance_detailed.5,
            })
        })
    }

    pub fn update_balance(
        edit_balance: EditBalance,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<BalanceDetailed> {
        use crate::schema::balance::dsl::*;
        connection.transaction(|conn| {
            let balance_detail = update(FilterDsl::filter(balance, id.eq(edit_balance.id)))
                .set(edit_balance)
                .get_result::<BalanceDetailed>(conn)?;
            Ok(balance_detail)
        })
    }

    pub fn list_balances(
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<Vec<DisplayBalanceDetailed>> {
        use crate::schema::account::dsl::{account, id as account_table_id, name as account_name};
        use crate::schema::balance::dsl::{
            account_id, amount, balance, entered_on, id as balance_table_id, memo,
        };
        connection.transaction(|conn| {
            let res = balance
                .inner_join(account.on(account_table_id.eq(account_id)))
                .select((
                    balance_table_id,
                    memo,
                    amount,
                    entered_on,
                    account_name,
                    account_table_id,
                ))
                .load::<(i32, String, f32, String, String, i32)>(conn)?;
            Ok(res
                .iter()
                .map(|val| DisplayBalanceDetailed {
                    id: val.0,
                    memo: val.1.clone(),
                    amount: val.2,
                    entered_on: val.3.clone(),
                    account_name: val.4.clone(),
                    account_id: val.5,
                })
                .collect())
        })
    }
}
