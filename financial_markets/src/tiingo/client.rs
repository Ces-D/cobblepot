use crate::{
    tiingo::model::{Metadata, MetadataRequest},
    url::BuildUrl,
};
use cobblepot_core::error::CobblepotResult;

use super::model::{HistoricalPricesRequest, PriceData};
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};

pub struct TiingoClient {
    client: Client,
}

impl TiingoClient {
    pub fn new(api_key: &str) -> reqwest::Result<Self> {
        let mut map = HeaderMap::new();
        map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let mut authorization_header = HeaderValue::from_str(api_key).expect("InvalidHeaderValue");
        authorization_header.set_sensitive(true);
        map.insert(AUTHORIZATION, authorization_header);
        map.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client = reqwest::ClientBuilder::new().default_headers(map).build()?;

        Ok(TiingoClient {
            client,
        })
    }

    pub async fn end_of_day_prices(
        &self,
        req: HistoricalPricesRequest,
    ) -> CobblepotResult<Vec<PriceData>> {
        let url = req.build_url().unwrap();
        let res = self.client.get(url).send().await?.json::<Vec<PriceData>>().await?;
        Ok(res)
    }

    pub async fn metadata(&self, req: MetadataRequest) -> CobblepotResult<Metadata> {
        let url = req.build_url().unwrap();
        let res = self.client.get(url).send().await?.json::<Metadata>().await?;
        Ok(res)
    }
}
