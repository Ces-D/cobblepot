use crate::serde::date_yyyy_mm_dd;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

const TIINGO_API_BASE_URL: &str = "https://api.tiingo.com/api/";
#[derive(Debug, Copy, Clone, Serialize, Deserialize, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum ResampleFrequency {
    Daily,
    Weekly,
    Monthly,
    Annually,
}

/// Metadata about a given ticker
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub ticker: String,
    pub name: String,
    pub exchange_code: String,
    pub start_date: String,
    pub end_date: String,
    pub frequency: String,
}

/// A single day’s end‑of‑day price record
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceData {
    pub date: DateTime<Utc>,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: u64,
    pub adj_close: f64,
    pub adj_high: f64,
    pub adj_low: f64,
    pub adj_open: f64,
    pub adj_volume: u64,
    pub div_cash: f64,
    pub split_factor: f64,
}

/// Parameters for fetching historical price data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPricesRequest {
    /// The equity ticker symbol, e.g. "AAPL"
    pub ticker: String,

    /// Inclusive start date, e.g. "2020-01-01". Exclude this to get the latest price data
    #[serde(skip_serializing_if = "Option::is_none", with = "date_yyyy_mm_dd")]
    pub start_date: Option<DateTime<Utc>>,

    /// Inclusive end date, e.g. "2020-12-31". Exclude this to get the latest price data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,

    /// Resampling frequency: "daily" | "weekly" | "monthly" (defaults to daily)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resample_freq: Option<ResampleFrequency>,

    /// Specify the sort direct and which column to sort by. Prepend "-" if you want descending order
    pub sort: Option<String>,

    /// Response format: "json" | "csv" (defaults to json)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub columns: Option<Vec<String>>,
}

impl HistoricalPricesRequest {
    pub fn build_url(&self) -> Url {
        let mut u = Url::parse(TIINGO_API_BASE_URL).unwrap();
        let path = vec!["daily", self.ticker.as_str(), "prices"];

        // Append each segment (percent‑encoded automatically)
        {
            let mut p = u.path_segments_mut().unwrap();
            for seg in path {
                p.push(seg);
            }
        }

        // Add query pairs (also percent‑encoded)
        {
            let mut qp = u.query_pairs_mut();
            if let Some(start_date) = self.start_date {
                qp.append_pair("start_date", date_yyyy_mm_dd::to_string(&start_date).as_str());
            }
            if let Some(end_date) = self.end_date {
                qp.append_pair("end_date", date_yyyy_mm_dd::to_string(&end_date).as_str());
            }
            if let Some(resample_freq) = self.resample_freq {
                qp.append_pair("resampleFreq", resample_freq.to_string().as_str());
            }
            if let Some(sort) = self.sort.clone() {
                qp.append_pair("sort", sort.as_str());
            }
            if let Some(format) = self.format.clone() {
                qp.append_pair("format", format.as_str());
            }
        }

        u
    }
}

/// Parameters for fetching ticker metadata
pub struct MetadataRequest {
    /// The equity ticker symbol, e.g. "AAPL"
    pub ticker: String,
}

impl MetadataRequest {
    pub fn build_url(&self) -> Url {
        let mut u = Url::parse(TIINGO_API_BASE_URL).unwrap();
        let path = vec!["daily", self.ticker.as_str()];

        // Append each segment (percent‑encoded automatically)
        {
            let mut p = u.path_segments_mut().unwrap();
            for seg in path {
                p.push(seg);
            }
        }

        u
    }
}
