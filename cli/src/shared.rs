use clap::ValueEnum;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

/// ValueEnum for AccountType
#[derive(
    Debug,
    ValueEnum,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    FromPrimitive,
    IntoPrimitive,
)]
#[repr(i32)]
pub enum AccountType {
    #[default]
    Asset = 0,
    Liability = 1,
}

// ValueEnum
#[derive(
    Debug,
    ValueEnum,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    FromPrimitive,
    IntoPrimitive,
)]
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
