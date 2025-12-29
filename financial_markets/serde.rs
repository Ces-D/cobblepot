/// Reusable serde module for `"YYYY-MM-DD"` format
pub mod date_yyyy_mm_dd {
    use chrono::{DateTime, Utc};
    use serde::Serializer;

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
}
