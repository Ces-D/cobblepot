use cobblepot_core::error::CobblepotResult;
use url::Url;

pub trait BuildUrl {
    fn build_url(&self) -> CobblepotResult<Url>;
}
