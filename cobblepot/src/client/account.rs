pub mod query {
    use crate::client::model::{AccountDetailed, EditAccount, NewAccount};
    use diesel::{
        Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, insert_into, update,
    };

    pub fn insert_new_account(
        new_account: NewAccount,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<AccountDetailed> {
        use crate::schema::account::dsl::*;
        connection.transaction(|conn| {
            let account_detail = insert_into(account)
                .values(new_account)
                .get_result::<AccountDetailed>(conn)?;
            Ok(account_detail)
        })
    }

    pub fn update_account(
        edit_account: EditAccount,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<AccountDetailed> {
        use crate::schema::account::dsl::*;
        connection.transaction(|conn| {
            let account_detail = update(account.filter(id.eq(edit_account.id)))
                .set(edit_account)
                .get_result::<AccountDetailed>(conn)?;
            Ok(account_detail)
        })
    }

    pub fn list_accounts(
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<Vec<AccountDetailed>> {
        use crate::schema::account::dsl::*;
        connection.transaction(|conn| account.load::<AccountDetailed>(conn))
    }
}
