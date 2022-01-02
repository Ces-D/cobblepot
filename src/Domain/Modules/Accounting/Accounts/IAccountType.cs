namespace Cobblepot.Domain.Accounting.Accounts;

public interface IAccountType
{
    Money Value { get; }
    AccountType AccountType { get; }
    void Credit(Money money);
    void Debit(Money money);
}
