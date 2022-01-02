namespace Cobblepot.Domain.Accounting.Accounts;

public class Liability : IAccountType
{
    private Money _value;

    public Liability(Currency currency)
    {
        _value = new Money(0, currency);
    }

    public Money Value => _value;
    public AccountType AccountType => AccountType.Liability;

    public void Credit(Money money) => _value += money;

    public void Debit(Money money) => _value -= money;
}