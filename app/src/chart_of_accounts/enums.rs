/// An accounting period is a specific period of time, typically a month, quarter, or year, during which a business or organization tracks and records its financial transactions.
pub enum AccountingPeriod {
    Yearly,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
}

/// The classification of the account as an asset, liability, equity, revenue, or expense account, which determines how the account is treated for financial reporting purposes
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}
