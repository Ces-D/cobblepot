use clap::Parser;
use diesel::{Insertable, Queryable, Selectable, prelude::AsChangeset, sqlite::Sqlite};

use super::shared::{
    ISO8601_DATE_LONG_HELP, default_iso8601_variant_date, parse_iso8601_variant_date,
};
use crate::schema::balance;

#[derive(Debug, Insertable, Parser)]
#[diesel(table_name=balance)]
#[command(about = "Add a new balance entry")]
pub struct NewBalance {
    #[arg(short, help = "ID of the account", required = true)]
    pub account_id: i32,
    #[arg(short, help = "Memo of the balance", required = true)]
    pub memo: String,
    #[arg(short, help = "Amount of the balance", required = true)]
    pub amount: f32,
    #[arg(short, help = "Date the balance is entered", long_help=ISO8601_DATE_LONG_HELP, required = true, default_value_t = default_iso8601_variant_date(), value_parser = parse_iso8601_variant_date)]
    pub entered_on: String,
}

#[derive(Debug, AsChangeset, Parser)]
#[diesel(table_name=balance)]
#[command(about = "Edit an existing balance entry")]
pub struct EditBalance {
    #[arg(short, help = "ID of the balance entry", required = true)]
    pub id: i32,
    #[arg(short, help = "Memo of the balance")]
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, serde::Serialize, tabled::Tabled)]
#[diesel(table_name=balance)]
#[diesel(check_for_backend(Sqlite))]
#[tabled(display(Option, "display::option", "Unknown"))]
pub struct BalanceDetailed {
    pub id: i32,
    pub memo: String,
    pub amount: f32,
    pub entered_on: String,
    pub account_id: i32,
}

pub mod query {
    use diesel::{
        Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, insert_into, update,
    };

    pub fn insert_new_balance(
        new_balance: super::NewBalance,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<super::BalanceDetailed> {
        use crate::schema::balance::dsl::*;
        connection.transaction(|conn| {
            let balance_detail = insert_into(balance)
                .values(new_balance)
                .get_result::<super::BalanceDetailed>(conn)?;
            Ok(balance_detail)
        })
    }

    pub fn update_balance(
        edit_balance: super::EditBalance,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<super::BalanceDetailed> {
        use crate::schema::balance::dsl::*;
        connection.transaction(|conn| {
            let balance_detail = update(balance.filter(id.eq(edit_balance.id)))
                .set(edit_balance)
                .get_result::<super::BalanceDetailed>(conn)?;
            Ok(balance_detail)
        })
    }

    pub fn list_balances(
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<Vec<super::BalanceDetailed>> {
        use crate::schema::balance::dsl::*;
        connection.transaction(|conn| balance.load::<super::BalanceDetailed>(conn))
    }
}
