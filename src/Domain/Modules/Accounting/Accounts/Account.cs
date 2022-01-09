namespace Cobblepot.Domain.Accounting.Accounts;
using Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public class Account : Entity
{
    private string _name;
    private Money _value;
    private IAccountValueCalculator _accountValueCalculator;

    public Account(string name, Currency currency, IAccountValueCalculator accountValueCalculator) : base(new AccountId(), DateTime.UtcNow)
    {
        _name = name;
        _value = new Money(0, currency);
        _accountValueCalculator = accountValueCalculator;
    }

    public string Name { get => _name; }
    public Money Value { get => _value; }

    public void Credit(Money money)
    {
        _value = _accountValueCalculator.Credit(_value, money);
    }

    public void Debit(Money money)
    {
        _value = _accountValueCalculator.Debit(_value, money);
    }

}