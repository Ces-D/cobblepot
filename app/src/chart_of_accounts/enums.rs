pub enum AccountingPeriod {
    Yearly,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
}

// The classification of the account as an asset, liability, equity, revenue, or expense account, which determines how the account is treated for financial reporting purposes
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}
