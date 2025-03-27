use clap::Parser;
use diesel::{
    Insertable, Queryable, Selectable,
    prelude::{AsChangeset, Identifiable},
    sqlite::Sqlite,
};
use tabled::derive::display;

use super::shared::{
    AccountType, ISO8601_DATE_LONG_HELP, default_iso8601_variant_date, parse_iso8601_variant_date,
};
use crate::schema::account;

#[derive(Debug, Insertable, serde::Deserialize, Parser)]
#[diesel(table_name=account)]
#[diesel(check_for_backend(Sqlite))]
#[command(about = "Add a new account")]
pub struct NewAccount {
    #[arg(short, help = "Name of the account", required = true)]
    pub name: String,
    #[arg(short, help = "Description of the account")]
    pub description: Option<String>,
    #[arg(short = 'w', help = "Owner of the account", default_value = "me")]
    pub owner: String,
    #[arg(short, help = "Type of the account", default_value = "asset")]
    pub account_type: Option<AccountType>,
    #[arg(short, help = "Date the account was opened", long_help=ISO8601_DATE_LONG_HELP, required = true, default_value_t = default_iso8601_variant_date(), value_parser = parse_iso8601_variant_date)]
    pub opened_on: String,
}

#[derive(Debug, AsChangeset, Parser, Identifiable)]
#[diesel(table_name=account)]
#[diesel(check_for_backend(Sqlite))]
#[command(about = "Edit an existing account")]
pub struct EditAccount {
    #[arg(short, help = "ID of the account", required = true)]
    pub id: i32,
    #[arg(short, help = "Name of the account")]
    pub name: Option<String>,
    #[arg(short, help = "Description of the account")]
    pub description: Option<String>,
    #[arg(short, help = "Owner of the account")]
    pub owner: Option<String>,
    #[arg(short, help = "Date the account was closed", long_help=ISO8601_DATE_LONG_HELP, value_parser = parse_iso8601_variant_date)]
    pub closed_on: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, serde::Serialize, tabled::Tabled)]
#[diesel(table_name=account)]
#[diesel(check_for_backend(Sqlite))]
#[tabled(display(Option, "display::option", "Unknown"))]
pub struct AccountDetailed {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: AccountType,
    pub opened_on: String,
    pub closed_on: Option<String>,
}

pub mod query {
    use diesel::{
        Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, insert_into, update,
    };

    pub fn insert_new_account(
        new_account: super::NewAccount,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<super::AccountDetailed> {
        use crate::schema::account::dsl::*;
        connection.transaction(|conn| {
            let account_detail = insert_into(account)
                .values(new_account)
                .get_result::<super::AccountDetailed>(conn)?;
            Ok(account_detail)
        })
    }

    pub fn update_account(
        edit_account: super::EditAccount,
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<super::AccountDetailed> {
        use crate::schema::account::dsl::*;
        connection.transaction(|conn| {
            let account_detail = update(account.filter(id.eq(edit_account.id)))
                .set(edit_account)
                .get_result::<super::AccountDetailed>(conn)?;
            Ok(account_detail)
        })
    }

    pub fn list_accounts(
        mut connection: diesel::SqliteConnection,
    ) -> QueryResult<Vec<super::AccountDetailed>> {
        use crate::schema::account::dsl::*;
        connection.transaction(|conn| account.load::<super::AccountDetailed>(conn))
    }
}
