#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountType {
    AssetCash,
    AssetSavings,
    AssetChecking,
    AssetInvestments,
    AssetRetirementAccounts,
    AssetAccountsReceivable,
    AssetInventory,
    AssetEquipment,
    AssetRealEstate,
    AssetVehicles,
    AssetPrepaidExpenses,
    AssetIntangibleAssets,
    AssetOther,

    LiabilityCreditCardDebt,
    LiabilityPersonalLoans,
    LiabilityBusinessLoans,
    LiabilityAccountsPayable,
    LiabilityMortgages,
    LiabilityCarLoans,
    LiabilityStudentLoans,
    LiabilityTaxesPayable,
    LiabilityWagesPayable,
    LiabilityUtilitiesPayable,
    LiabilityRentPayable,
    LiabilityDeferredRevenue,
    LiabilityAccruedExpenses,
    LiabilityOther,

    EquityOwnersEquity,
    EquityRetainedEarnings,
    EquityCapitalContributions,
    EquityCommonStock,
    EquityPreferredStock,
    EquityAdditionalPaidInCapital,
    EquityTreasuryStock,
    EquityDividends,
    EquityAccumulatedOtherComprehensiveIncome,
    EquityOther,

    IncomeSalary,
    IncomeBusinessRevenue,
    IncomeFreelanceIncome,
    IncomeInvestmentIncome,
    IncomeRentalIncome,
    IncomeInterestIncome,
    IncomeDividendIncome,
    IncomeCapitalGains,
    IncomeRoyalties,
    IncomeGrants,
    IncomeOther,

    ExpenseRent,
    ExpenseUtilities,
    ExpenseSalaries,
    ExpenseWages,
    ExpenseSupplies,
    ExpenseInsurance,
    ExpenseMarketing,
    ExpenseTravel,
    ExpenseMealsAndEntertainment,
    ExpenseOfficeExpenses,
    ExpenseLegalFees,
    ExpenseAccountingFees,
    ExpenseDepreciation,
    ExpenseTaxes,
    ExpenseInterestExpense,
    ExpenseCostOfGoodsSold,
    ExpenseMaintenanceAndRepairs,
    ExpenseSubscriptions,
    ExpenseOther,
}

impl AccountType {
    pub fn is_asset(t: AccountType) -> bool {
        match t {
            AccountType::AssetCash => true,
            AccountType::AssetSavings => true,
            AccountType::AssetChecking => true,
            AccountType::AssetInvestments => true,
            AccountType::AssetRetirementAccounts => true,
            AccountType::AssetAccountsReceivable => true,
            AccountType::AssetInventory => true,
            AccountType::AssetEquipment => true,
            AccountType::AssetRealEstate => true,
            AccountType::AssetVehicles => true,
            AccountType::AssetPrepaidExpenses => true,
            AccountType::AssetIntangibleAssets => true,
            AccountType::AssetOther => true,
            _ => false,
        }
    }

    pub fn is_liability(t: AccountType) -> bool {
        match t {
            AccountType::LiabilityCreditCardDebt => true,
            AccountType::LiabilityPersonalLoans => true,
            AccountType::LiabilityBusinessLoans => true,
            AccountType::LiabilityAccountsPayable => true,
            AccountType::LiabilityMortgages => true,
            AccountType::LiabilityCarLoans => true,
            AccountType::LiabilityStudentLoans => true,
            AccountType::LiabilityTaxesPayable => true,
            AccountType::LiabilityWagesPayable => true,
            AccountType::LiabilityUtilitiesPayable => true,
            AccountType::LiabilityRentPayable => true,
            AccountType::LiabilityDeferredRevenue => true,
            AccountType::LiabilityAccruedExpenses => true,
            AccountType::LiabilityOther => true,
            _ => false,
        }
    }

    pub fn is_equity(t: AccountType) -> bool {
        match t {
            AccountType::EquityOwnersEquity => true,
            AccountType::EquityRetainedEarnings => true,
            AccountType::EquityCapitalContributions => true,
            AccountType::EquityCommonStock => true,
            AccountType::EquityPreferredStock => true,
            AccountType::EquityAdditionalPaidInCapital => true,
            AccountType::EquityTreasuryStock => true,
            AccountType::EquityDividends => true,
            AccountType::EquityAccumulatedOtherComprehensiveIncome => true,
            AccountType::EquityOther => true,
            _ => false,
        }
    }

    pub fn is_income(t: AccountType) -> bool {
        match t {
            AccountType::IncomeSalary => true,
            AccountType::IncomeBusinessRevenue => true,
            AccountType::IncomeFreelanceIncome => true,
            AccountType::IncomeInvestmentIncome => true,
            AccountType::IncomeRentalIncome => true,
            AccountType::IncomeInterestIncome => true,
            AccountType::IncomeDividendIncome => true,
            AccountType::IncomeCapitalGains => true,
            AccountType::IncomeRoyalties => true,
            AccountType::IncomeGrants => true,
            AccountType::IncomeOther => true,
            _ => false,
        }
    }

    pub fn is_expense(t: AccountType) -> bool {
        match t {
            AccountType::ExpenseRent => true,
            AccountType::ExpenseUtilities => true,
            AccountType::ExpenseSalaries => true,
            AccountType::ExpenseWages => true,
            AccountType::ExpenseSupplies => true,
            AccountType::ExpenseInsurance => true,
            AccountType::ExpenseMarketing => true,
            AccountType::ExpenseTravel => true,
            AccountType::ExpenseMealsAndEntertainment => true,
            AccountType::ExpenseOfficeExpenses => true,
            AccountType::ExpenseLegalFees => true,
            AccountType::ExpenseAccountingFees => true,
            AccountType::ExpenseDepreciation => true,
            AccountType::ExpenseTaxes => true,
            AccountType::ExpenseInterestExpense => true,
            AccountType::ExpenseCostOfGoodsSold => true,
            AccountType::ExpenseMaintenanceAndRepairs => true,
            AccountType::ExpenseSubscriptions => true,
            AccountType::ExpenseOther => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountCode {
    id: u16,
    pub account_type: AccountType,
}
