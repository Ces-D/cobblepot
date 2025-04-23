use clap::Parser;
use diesel::{
    Insertable, Queryable, Selectable,
    prelude::{AsChangeset, Identifiable, QueryableByName},
    sqlite::Sqlite,
};
use tabled::derive::display;

use super::shared::{
    cli::{ISO8601_DATE_LONG_HELP, default_iso8601_variant_date, parse_iso8601_variant_date},
    sql::AccountType,
};
use crate::schema::{account, balance};

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
    #[arg(short, help = "Date the account was opened", long_help=ISO8601_DATE_LONG_HELP, default_value_t = default_iso8601_variant_date(0), value_parser = parse_iso8601_variant_date)]
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

#[derive(
    Debug, Clone, Queryable, QueryableByName, Selectable, serde::Serialize, tabled::Tabled,
)]
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

#[derive(Debug, Insertable, Parser)]
#[diesel(table_name=balance)]
#[command(about = "Add an updated balance entry to an account")]
pub struct NewBalance {
    #[arg(short = 'i', help = "ID of the account", required = true)]
    pub account_id: i32,
    #[arg(short, help = "Memo for this balance", default_value = "")]
    pub memo: String,
    #[arg(short = 'a', help = "Amount of the balance", required = true)]
    pub amount: f32,
    #[arg(short, help = "Date the balance is entered", long_help=ISO8601_DATE_LONG_HELP, default_value_t = default_iso8601_variant_date(0), value_parser = parse_iso8601_variant_date)]
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

#[derive(
    Debug, Clone, Queryable, QueryableByName, Selectable, serde::Serialize, tabled::Tabled,
)]
#[diesel(table_name=balance)]
#[diesel(check_for_backend(Sqlite))]
pub struct BalanceDetailed {
    pub id: i32,
    pub memo: String,
    pub amount: f32,
    pub entered_on: String,
    pub account_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, tabled::Tabled)]
#[tabled(display(Option, "display::option", "Unknown"))]
pub struct DisplayBalanceDetailed {
    pub id: i32,
    pub memo: String,
    pub amount: f32,
    pub entered_on: String,
    pub account_name: String,
    pub account_id: i32,
}
