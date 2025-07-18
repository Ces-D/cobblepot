/// Reusable serde module for `"YYYY-MM-DD"` format
pub mod date_yyyy_mm_dd {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn to_string(dt: &DateTime<Utc>) -> String {
        dt.format(FORMAT).to_string()
    }

    pub fn serialize<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(dt) = dt {
            let s = dt.format(FORMAT).to_string();
            serializer.serialize_str(&s)
        } else {
            Err(serde::ser::Error::custom("This should skip serialization if None"))
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // parse as a NaiveDate then assume UTC midnight
        let nd = chrono::NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(Utc.from_utc_datetime(&nd.and_hms_opt(0, 0, 0).unwrap()))
    }
}
