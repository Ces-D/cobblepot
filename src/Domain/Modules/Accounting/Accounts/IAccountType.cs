namespace Cobblepot.Domain.Accounting.Accounts;

using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public interface IAccountType
{
    Money Value { get; }
    AccountType AccountType { get; }
    void Credit(Money money);
    void Debit(Money money);
}
