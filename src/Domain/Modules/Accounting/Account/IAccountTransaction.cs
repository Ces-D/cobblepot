namespace Cobblepot.Domain.Accounting.Account;

public interface IAccountTransaction
{
    DateTime TransactionDate { get; }
    bool IsCredit { get; }
    Money Amount { get; }
    AccountType AccountType { get; }
}
