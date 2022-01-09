namespace Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public struct AssetValueCalculator : IAccountValueCalculator
{
    public Money Credit(Money currentValue, Money credit) => currentValue -= credit;

public Money Debit(Money currentValue, Money debit) => currentValue += debit;
}