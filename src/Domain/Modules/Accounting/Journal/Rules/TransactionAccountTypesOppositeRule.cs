namespace Cobblepot.Domain.Accounting.Journal;
using Cobblepot.Domain.Accounting.Account;

internal record TransactionAccountTypesOppositeRule : IBusinessRule
{
    private bool _accountTypesOpposite;

    public TransactionAccountTypesOppositeRule(IAccountTransaction initialTransaction, IAccountTransaction secondTransaction)
    {
        if (initialTransaction.AccountType == AccountType.Asset)
        {
            _accountTypesOpposite = secondTransaction.AccountType == AccountType.Liability;
        }
        else if (initialTransaction.AccountType == AccountType.Liability)
        {
            _accountTypesOpposite = secondTransaction.AccountType == AccountType.Asset;
        }
        else if (initialTransaction.AccountType == AccountType.Revenue)
        {
            _accountTypesOpposite = secondTransaction.AccountType == AccountType.Expense;
        }
        else if (initialTransaction.AccountType == AccountType.Expense)
        {
            _accountTypesOpposite = secondTransaction.AccountType == AccountType.Revenue;
        }
        else { _accountTypesOpposite = false; }
    }

    public string Message => "Transaction account types must handle credits and debits inversely";

    public bool IsBroken() => _accountTypesOpposite;
}