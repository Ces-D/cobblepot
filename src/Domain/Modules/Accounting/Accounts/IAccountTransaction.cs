namespace Cobblepot.Domain.Accounting.Accounts;

public interface IAccountTransaction
{
    string Title { get; }
    string Memo { get; }
    bool IsCredit { get; }
    DateTime TransactionDate { get; }
    Money Amount { get; }
    AccountType AccountType { get; }
}
