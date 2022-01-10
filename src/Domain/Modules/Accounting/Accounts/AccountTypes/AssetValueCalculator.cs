namespace Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public struct AssetValueCalculator : IAccountValueCalculator
{
    public AccountType AccountType => AccountType.Asset;
    public Money Credit(Money currentValue, Money credit) => currentValue -= credit;

    public Money Debit(Money currentValue, Money debit) => currentValue += debit;
}