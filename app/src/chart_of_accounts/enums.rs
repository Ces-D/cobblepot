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

pub enum AccountDivision {
    Asset(),
    Liability(),
    Equity(),
    Revenue(),
    Expense(),
}

pub enum PersonalAsset {
    Cash,
    CheckingAccount,
    SavingsAccount,
    EmergencyFund,
    CertificatesOfDeposit,
    MoneyMarketAccount,
    InvestmentAccounts,
    /// A retirement account is a tax-advantaged investing tool that individuals use to earmark funds for retirement savings. There are several types of retirement plans including 401(k)s and individual retirement accounts (IRAs). IRA, 401k, 403b,
    RetirementAccounts,
    /// (Primary_Residence, Rental Property, etc.
    RealEstate,
    /// (Cars, Motorcycles, Boats, etc.)
    Vehicles,
    /// (Art, Antiques, Coins, etc.)
    Collectibles,
    ///  (Jewelry, Electronics, Furniture, etc.)
    PersonalProperty,
}

pub enum PersonalLiability {
    CreditCardDebt,
    PersonalLoans,
    StudentLoans,
    AutoLoans,
    Mortgage,
    HomeEquityLoan,
    LineOfCredit,
    MedicalDebt,
    TaxesOwed,
    OtherOutstandingDebts,
}

pub enum PersonalEquity {
    OwnershipEquity,
}

pub enum PersonalRevenue {
    Salary,
    HourlyWages,
    Bonuses,
    Commissions,
    DividendsAndInterestIncome,
    RentalIncome,
    RoyaltyIncome,
    SocialSecurityBenefits,
    Pension,
    UnemploymentBenefits,
    GiftsAndInheritances,
    MiscellaneousIncome,
}

pub enum PersonalExpense {
    RentOrMortgage,
    /// (Electricity, Water, Gas, etc.)
    Utilities,
    Groceries,
    DiningOut,
    /// (Fuel, Public Transit, Tolls, etc.)
    Transportation,
    /// (Health, Auto, Home, Life, etc.)
    Insurance,
    ///  (Doctor Visits, Prescriptions, etc.)
    MedicalExpenses,
    /// (Credit Cards, Loans, etc.)
    DebtPayments,
    /// (Income, Property, etc.)
    Taxes,
    SavingsAndInvestmentsContributions,
    /// (Movies, Concerts, etc.)
    Entertainment,
    TravelAndVacations,
    ClothingAndApparel,
    /// (Tuition, Books, etc.)
    Education,
    CharitableDonations,
    MiscellaneousExpenses,
}

pub enum BusinessAsset {
    Cash,
    CheckingAccount,
    SavingsAccount,
    AccountsReceivable,
    Inventory,
    PrepaidExpenses,
    ///  (Machinery, Equipment, Furniture, etc.)
    FixedAssets,
    AccumulatedDepreciation,
    Vehicles,
    Buildings,
    Land,
    LeaseholdImprovements,
    AccumulatedAmortization,
    /// (Patents, Trademarks, Licenses, etc.)
    IntangibleAssets,
    SecurityDeposits,
}

pub enum BusinessLiabilities {
    AccountsPayable,
    AccruedExpenses,
    ShortTermLoans,
    LineOfCredit,
    LongTermLoans,
    SalesTaxPayable,
    PayrollTaxesPayable,
    IncomeTaxesPayable,
    DeferredRevenue,
    CustomerDeposits,
}

pub enum BusinessEquity {
    OwnersCapital,
    OwnersDrawings,
    RetainedEarnings,
}

pub enum BusinessRevenue {
    SalesRevenue,
    ServiceRevenue,
    InterestIncome,
    RentalIncome,
    MiscellaneousIncome,
}

pub enum BusinessExpense {
    CostOfGoodsSold,
    AdvertisingAndMarketing,
    RentOrLeaseExpense,
    /// (Electricity, Water, Gas, etc.)
    Utilities,
    SalariesAndWages,
    PayrollTaxes,
    EmployeeBenefits,
    /// (General Liability, Property, etc.)
    Insurance,
    DepreciationAndAmortization,
    OfficeSupplies,
    TelephoneAndInternet,
    PostageAndShipping,
    TravelAndEntertainment,
    /// (Accounting, Legal, etc.)
    ProfessionalFees,
    BankServiceCharges,
    InterestExpense,
    TaxesAndLicenses,
    RepairsAndMaintenance,
    BadDebtExpense,
}
