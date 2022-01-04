namespace Cobblepot.Domain.Accounting.Accounts;

using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public interface IAccountTransaction
{
    string Title { get; }
    string Memo { get; }
    bool IsCredit { get; }
    DateTime TransactionDate { get; }
    Money Amount { get; }
    AccountType AccountType { get; }
}
