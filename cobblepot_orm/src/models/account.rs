use diesel::expression::AsExpression;
use diesel::prelude::*;

#[derive(AsExpression, Debug, PartialEq, Clone, Copy)]
#[diesel(sql_type=diesel::sql_types::Text)]
pub enum AccountVariant {
    Asset,
    Liability,
}

impl From<String> for AccountVariant {
    fn from(value: String) -> Self {
        if value == String::from("Asset") {
            AccountVariant::Asset
        } else if value == String::from("Liability") {
            AccountVariant::Liability
        } else {
            panic!("From value: {} is not a valid AccountVariant", value)
        }
    }
}

impl From<AccountVariant> for String {
    fn from(value: AccountVariant) -> Self {
        match value {
            AccountVariant::Asset => String::from("Asset"),
            AccountVariant::Liability => String::from("Liability"),
        }
    }
}

#[derive(Queryable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    account_code: i32,
    #[diesel(deserialize_as = String)]
    account_variant: AccountVariant,
    pub name: String,
    pub description: Option<String>,
    created_on: chrono::NaiveDateTime,
    closed_on: Option<chrono::NaiveDateTime>,
}

impl Account {
    pub fn account_code(&self) -> i32 {
        self.account_code.clone()
    }
    pub fn created_on(&self) -> chrono::NaiveDateTime {
        self.created_on.clone()
    }
    pub fn closed_on(&self) -> Option<chrono::NaiveDateTime> {
        if self.closed_on.is_none() {
            None
        } else {
            Some(self.created_on.clone())
        }
    }
    pub fn account_variant(&self) -> AccountVariant {
        self.account_variant
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAccount {
    #[diesel(serialize_as=String)]
    account_variant: AccountVariant,
    name: String,
    description: Option<String>,
}
