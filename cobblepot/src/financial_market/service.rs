use actix_web::{Scope, web};
use diesel::{RunQueryDsl, dsl::insert_into};

use crate::{
    financial_market::model::{
        CalculatedMarketInstrument, InsertableMarketInstrument, JSONOpenFinancialMarketInstrument,
        MarketInstrument,
    },
    infrastructure::database::DbPool,
    schema::market_instrument::dsl::market_instrument,
};
use cobblepot_core::{environment::financial_api_key, error::CobblepotResult};
use cobblepot_financial_markets::tiingo::{client::TiingoClient, model::HistoricalPricesRequest};

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
            let api_key = financial_api_key()?;
            let client = TiingoClient::new(api_key.as_str())?;
            let prices = client
                .end_of_day_prices(HistoricalPricesRequest {
                    ticker: calculated_instrument.ticker.clone(),
                    start_date: None,
                    end_date: None,
                    resample_freq: None,
                    sort: None,
                    format: None,
                })
                .await?;

            if let Some(latest_price) = prices.first() {
                calculated_instrument.total_value =
                    calculated_instrument.quantity as f64 * latest_price.close;
            }

            Ok(calculated_instrument)
        }
        Err(e) => Err(e),
    }
}

pub fn financial_market_scope() -> Scope {
    web::scope("/financial_market").route("/open", web::post().to(insert_new_financial_instrument))
}
