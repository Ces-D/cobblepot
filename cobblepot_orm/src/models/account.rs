use chrono::{DateTime, Utc};
use diesel::prelude::*;

pub enum AccountVariant {
    Asset,
    Liability,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    account_code: i32,
    account_variant: String,
    pub name: String,
    pub description: Option<String>,
    created_on: String,
    closed_on: Option<String>,
}

impl Account {
    pub fn account_code(&self) -> i32 {
        self.account_code.clone()
    }
    pub fn created_on(&self) -> DateTime<Utc> {
        self.created_on.parse().unwrap()
    }
    pub fn closed_on(&self) -> Option<DateTime<Utc>> {
        if self.closed_on.is_none() {
            None
        } else {
            let dt: DateTime<Utc> = self.created_on.parse().unwrap();
            Some(dt)
        }
    }
    pub fn account_variant(&self) -> AccountVariant {
        match &self.account_variant {
            asset => AccountVariant::Asset,
            liability => AccountVariant::Liability,
            _ => panic!("Unidentified account variant"),
        }
    }
}
