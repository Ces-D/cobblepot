use crate::{
    account::model::{
        Account, ClosableAccount, InsertableAccount, JSONCloseAccount, JSONOpenAccount,
        JSONUpdateAccount, UpdatableAccount,
    },
    infrastructure::database::DbPool,
    schema::account::dsl::{account, id},
    shared::CobblepotResult,
};
use actix_web::{HttpResponse, web};
use chrono::Utc;
use diesel::{ExpressionMethods, RunQueryDsl, insert_into, query_dsl::methods::FilterDsl, update};

pub async fn create_account(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenAccount>,
) -> CobblepotResult<Account> {
    let args = payload.into_inner();
    let acc = web::block(move || {
        let mut conn = pool.get().unwrap();
        let insertable: InsertableAccount = args.into();
        let res = insert_into(account).values(insertable).get_result::<Account>(&mut conn)?;
        Ok(res)
    })
    .await?;
    acc
}

pub async fn update_account(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONUpdateAccount>,
) -> CobblepotResult<Account> {
    let args = payload.into_inner();
    let acc = web::block(move || {
        let mut conn = pool.get().unwrap();
        let updateable: UpdatableAccount = args.into();
        let res = update(account).set(updateable).get_result::<Account>(&mut conn)?;
        Ok(res)
    })
    .await?;
    acc
}

pub async fn close_account(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONCloseAccount>,
) -> CobblepotResult<HttpResponse> {
    let args = payload.into_inner();
    let acc = web::block(move || {
        let mut conn = pool.get().unwrap();
        let closable: ClosableAccount = if args.closed_on.is_some() {
            args.into()
        } else {
            ClosableAccount {
                id: args.id,
                closed_on: Some(Utc::now().naive_utc()),
            }
        };
        update(account.filter(id.eq(args.id))).set(closable).execute(&mut conn)
    })
    .await?;
    if acc.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}
