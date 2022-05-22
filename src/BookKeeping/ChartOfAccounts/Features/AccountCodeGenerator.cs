using System;
using Cobblepot.BookKeeping.ChartOfAccounts.ChartOfAccount;
using Cobblepot.BookKeeping.ChartOfAccounts.Data;
namespace Cobblepot.BookKeeping.ChartOfAccounts.Features;

public class AccountCodeGenerator
{
    private readonly int _assetMinCode = 10_000;
    private readonly int _assetMaxCode = 19_999;
    private readonly int _liabilityMinCode = 20_000;
    private readonly int _liabilityMaxCode = 29_999;
    private readonly int _equityMinCode = 30_000;
    private readonly int _equityMaxCode = 39_999;
    private readonly int _revenueMinCode = 40_000;
    private readonly int _revenueMaxCode = 49_999;
    private readonly int _expenseMinCode = 50_000;
    private readonly int _expenseMaxCode = 59_999;

    public AccountCode CreateNew(AccountGroup accountGroup, Enum SubAccountGroup, byte departmentCode)
    {
        int accountCode = accountGroup switch
        {
            AccountGroup.Asset => this._assetMinCode,
            AccountGroup.Liability => this._liabilityMinCode,
            AccountGroup.Equity => this._equityMinCode,
            AccountGroup.Revenue => this._revenueMinCode,
            AccountGroup.Expense => this._expenseMinCode,
            _ => 0
        };


        return new AccountCode(departmentCode, accountCode);
    }
}