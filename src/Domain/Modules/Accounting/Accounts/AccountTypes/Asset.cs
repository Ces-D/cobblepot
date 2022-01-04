using Cobblepot.Domain.Accounting.Accounts;

namespace Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public class Asset : IAccountType
{
    private Money _value;

    public Asset(Currency currency)
    {
        _value = new Money(0, currency);
    }

    public Money Value => _value;
    public AccountType AccountType => AccountType.Asset;

    public void Credit(Money money) => _value -= money;

    public void Debit(Money money) => _value += money;
}