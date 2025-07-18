use url::{ParseError, Url};

pub trait BuildUrl {
    fn build_url(&self) -> Result<Url, ParseError>;
}
