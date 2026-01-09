use chrono::{DateTime, NaiveDateTime};
use diesel::{
    AsExpression, FromSqlRow,
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Integer,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

/// A wrapper type that converts between NaiveDateTime and UNIX timestamp (Integer in SQLite)
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Integer)]
pub struct UnixTimestamp(pub NaiveDateTime);

impl UnixTimestamp {
    pub fn new(dt: NaiveDateTime) -> Self {
        UnixTimestamp(dt)
    }

    pub fn inner(&self) -> NaiveDateTime {
        self.0
    }
}

impl<DB> FromSql<Integer, DB> for UnixTimestamp
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let timestamp = i32::from_sql(bytes)?;
        let dt = DateTime::from_timestamp(timestamp as i64, 0)
            .ok_or_else(|| "Invalid timestamp")?
            .naive_utc();
        Ok(UnixTimestamp(dt))
    }
}

impl ToSql<Integer, Sqlite> for UnixTimestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let timestamp = self.0.and_utc().timestamp() as i32;
        out.set_value(timestamp);
        Ok(serialize::IsNull::No)
    }
}
