use crate::{
    balance::model::{
        Balance, InsertableBalance, JSONOpenBalance, JSONUpdateBalance, UpdatableBalance,
    },
    infrastructure::database::DbPool,
    schema::balance::dsl::balance,
    shared::CobblepotResult,
};
use actix_web::web;
use diesel::{RunQueryDsl, insert_into, update};

pub async fn insert_new_balance(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenBalance>,
) -> CobblepotResult<Balance> {
    let args = payload.into_inner();
    let bal = web::block(move || {
        let mut conn = pool.get().unwrap();
        let insertable: InsertableBalance = args.into();
        let res = insert_into(balance).values(insertable).get_result::<Balance>(&mut conn)?;
        Ok(res)
    })
    .await?;
    bal
}

pub async fn update_balance(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONUpdateBalance>,
) -> CobblepotResult<Balance> {
    let args = payload.into_inner();
    let bal = web::block(move || {
        let mut conn = pool.get().unwrap();
        let updatable: UpdatableBalance = args.into();
        let res = update(balance).set(updatable).get_result::<Balance>(&mut conn)?;
        Ok(res)
    })
    .await?;
    bal
}
