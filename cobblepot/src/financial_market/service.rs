use actix_web::{Scope, web};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, dsl::insert_into, update};

use crate::{
    financial_market::model::{
        CalculatedMarketInstrument, InsertableMarketInstrument, JSONOpenFinancialMarketInstrument,
        JSONUpdateFinancialMarketInstrument, MarketInstrument, MarketInstrumentList,
        UpdatableMarketInstrument,
    },
    infrastructure::database::DbPool,
    schema::market_instrument::dsl::{id, market_instrument},
};
use cobblepot_core::{environment::financial_api_key, error::CobblepotResult};
use cobblepot_financial_markets::tiingo::{client::TiingoClient, model::HistoricalPricesRequest};

pub async fn get_current_market_value(ticker: String, quantity: f32) -> CobblepotResult<f64> {
    let api_key = financial_api_key()?;
    let client = TiingoClient::new(api_key.as_str())?;
    let prices = client
        .end_of_day_prices(HistoricalPricesRequest {
            ticker,
            start_date: None,
            end_date: None,
            resample_freq: None,
            sort: None,
            format: None,
        })
        .await?;

    if let Some(latest_price) = prices.first() {
        Ok(quantity as f64 * latest_price.close)
    } else {
        Ok(0.0)
    }
}

async fn list_financial_instruments(
    pool: web::Data<DbPool>,
) -> CobblepotResult<MarketInstrumentList> {
    let mut conn = pool.get().unwrap();
    let res = market_instrument.load::<MarketInstrument>(&mut conn)?;
    Ok(MarketInstrumentList(res))
}

async fn insert_new_financial_instrument(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenFinancialMarketInstrument>,
) -> CobblepotResult<CalculatedMarketInstrument> {
    let args = payload.into_inner();
    let acc: CobblepotResult<MarketInstrument> = web::block(move || {
        let mut conn = pool.get().unwrap();
        let insertable: InsertableMarketInstrument = args.into();
        let res = insert_into(market_instrument)
            .values(insertable)
            .get_result::<MarketInstrument>(&mut conn)?;
        Ok(res)
    })
    .await?;

    match acc {
        Ok(instrument) => {
            let mut calculated_instrument: CalculatedMarketInstrument = instrument.into();
            calculated_instrument.total_value = get_current_market_value(
                calculated_instrument.ticker.clone(),
                calculated_instrument.quantity,
            )
            .await?;

            Ok(calculated_instrument)
        }
        Err(e) => Err(e),
    }
}

async fn update_financial_instrument(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONUpdateFinancialMarketInstrument>,
) -> CobblepotResult<CalculatedMarketInstrument> {
    let args = payload.into_inner();
    let instrument_id = args.id;
    let acc: CobblepotResult<MarketInstrument> = web::block(move || {
        let mut conn = pool.get().unwrap();
        let updatable: UpdatableMarketInstrument = args.into();
        let res = update(market_instrument.filter(id.eq(instrument_id)))
            .set(updatable)
            .get_result::<MarketInstrument>(&mut conn)?;
        Ok(res)
    })
    .await?;
    match acc {
        Ok(instrument) => {
            let mut calculated_instrument: CalculatedMarketInstrument = instrument.into();
            calculated_instrument.total_value = get_current_market_value(
                calculated_instrument.ticker.clone(),
                calculated_instrument.quantity,
            )
            .await?;
            Ok(calculated_instrument)
        }
        Err(e) => Err(e),
    }
}

pub fn financial_market_scope() -> Scope {
    web::scope("/financial_market")
        .route("/list", web::get().to(list_financial_instruments))
        .route("/open", web::post().to(insert_new_financial_instrument))
        .route("/update", web::post().to(update_financial_instrument))
}
