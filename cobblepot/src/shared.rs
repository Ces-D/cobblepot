use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, FromPrimitive, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum AccountType {
    #[default]
    Asset = 0,
    Liability = 1,
}

/// All the typical (and some exotic) instrument types an investor might own.
#[derive(Clone, Copy, Debug, FromPrimitive, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum InstrumentType {
    #[default]
    Stock = 0, // Equities & Funds
    Bond = 1,                      // Fixed Income & Money Market
    ETF = 2,                       // Equities & Funds
    MutualFund = 3,                // Equities & Funds
    Cryptocurrency = 4,            // Currencies & Commodities
    Commodity = 5,                 // Currencies & Commodities
    Option = 6,                    // Derivatives
    Future = 7,                    // Derivatives
    UnitInvestmentTrust = 8,       // Equities & Funds
    RealEstateInvestmentTrust = 9, // Equities & Funds
    ExchangeTradedNote = 10,       // Equities & Funds
    DepositaryReceipt = 11,        // Equities & Funds
    CertificateOfDeposit = 12,     // Fixed Income & Money Market
    MoneyMarketFund = 13,          // Fixed Income & Money Market
    CommercialPaper = 14,          // Fixed Income & Money Market
    MortgageBackedSecurity = 15,   // Fixed Income & Money Market
    AssetBackedSecurity = 16,      // Fixed Income & Money Market
    Currency = 17,                 // Currencies & Commodities
    Forward = 18,                  // Derivatives
    Swap = 19,                     // Derivatives
    CreditDefaultSwap = 20,        // Derivatives
    InterestRateSwap = 21,         // Derivatives
    Warrant = 22,                  // Hybrid & Structured Products
    StructuredProduct = 23,        // Hybrid & Structured Products
    ContractForDifference = 24,    // Hybrid & Structured Products
    HedgeFund = 25,                // Alternative & Private
    PrivateEquity = 26,            // Alternative & Private
    VentureCapitalFund = 27,       // Alternative & Private
    RealEstate = 28,               // Alternative & Private
    Collectible = 29,              // Alternative & Private
    Annuity = 30,                  // Insurance & Annuities
    P2PLoan = 31,                  // Others
}

#[derive(Clone, Copy, Debug, FromPrimitive, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum Frequency {
    Yearly = 0,
    #[default]
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Hourly = 4,
    Minutely = 5,
    Secondly = 6,
}

impl From<Frequency> for rrule::Frequency {
    fn from(val: Frequency) -> Self {
        match val {
            Frequency::Yearly => rrule::Frequency::Yearly,
            Frequency::Monthly => rrule::Frequency::Monthly,
            Frequency::Weekly => rrule::Frequency::Weekly,
            Frequency::Daily => rrule::Frequency::Daily,
            Frequency::Hourly => rrule::Frequency::Hourly,
            Frequency::Minutely => rrule::Frequency::Minutely,
            Frequency::Secondly => rrule::Frequency::Secondly,
        }
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum Weekday {
    #[default]
    Mon = 0,
    Tue = 1,
    Wed = 2,
    Thu = 3,
    Fri = 4,
    Sat = 5,
    Sun = 6,
}

impl From<Weekday> for rrule::Weekday {
    fn from(val: Weekday) -> Self {
        match val {
            Weekday::Mon => rrule::Weekday::Mon,
            Weekday::Tue => rrule::Weekday::Tue,
            Weekday::Wed => rrule::Weekday::Wed,
            Weekday::Thu => rrule::Weekday::Thu,
            Weekday::Fri => rrule::Weekday::Fri,
            Weekday::Sat => rrule::Weekday::Sat,
            Weekday::Sun => rrule::Weekday::Sun,
        }
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, Serialize, Deserialize)]
#[repr(i32)]
pub enum RecurringStatus {
    #[default]
    Ongoing = 0,
    Completed = 1,
    Closed = 2,
}

#[derive(Clone, Copy, Debug, FromPrimitive, Serialize, Deserialize)]
#[repr(i32)]
pub enum ReportType {
    #[default]
    BalanceSheet = 0,
    DeepDiveAccount = 1,
}

pub mod responder {

    /// Macro to implement Responder for types that should be serialized as JSON
    macro_rules! impl_json_responder {
        ($type:ty) => {
            impl Responder for $type {
                type Body = BoxBody;

                fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
                    match serde_json::to_string(&self) {
                        Ok(body) => HttpResponse::Ok().content_type(ContentType::json()).body(body),
                        Err(_) => HttpResponse::InternalServerError()
                            .content_type(ContentType::plaintext())
                            .body("Failed to serialize response"),
                    }
                }
            }
        };

        // For wrapper types like Lists that need to access .0
        ($type:ty, inner) => {
            impl Responder for $type {
                type Body = BoxBody;

                fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
                    match serde_json::to_string(&self.0) {
                        Ok(body) => HttpResponse::Ok().content_type(ContentType::json()).body(body),
                        Err(_) => HttpResponse::InternalServerError()
                            .content_type(ContentType::plaintext())
                            .body("Failed to serialize response"),
                    }
                }
            }
        };
    }

    pub(crate) use impl_json_responder;
}
