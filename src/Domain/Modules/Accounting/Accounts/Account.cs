namespace Cobblepot.Domain.Accounting.Accounts;

public class Account : Entity, IAccount
{
    private string _name;
    private Money _value;
    private IAccountType _accountType;

    public Account(string name, Currency currency, IAccountType accountType) : base(Guid.NewGuid(), DateTime.UtcNow)
    {
        _name = name;
        _value = new Money(0, currency);
        _accountType = accountType;
    }

    public string Name { get => _name; }
    public Money Value { get => _value; }
    public IAccountType AccountType { get => _accountType; }

    public void Credit(Money money) { }
    public void Debit(Money money) { }

}