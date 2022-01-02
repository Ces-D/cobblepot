namespace Cobblepot.Domain.Accounting.Accounts;

public class Revenue : IAccountType
{
    private Money _value;
    public Revenue(Currency currency)
    {
        _value = new Money(0, currency);
    }

    public Money Value => _value;
    public AccountType AccountType => AccountType.Asset;

    public void Credit(Money money) => _value += money;

    public void Debit(Money money) => _value -= money;
}