namespace Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public struct LiabilityValueCalculator : IAccountValueCalculator
{
    public AccountType AccountType => AccountType.Liability;

    public Money Credit(Money currentValue, Money credit) => currentValue += credit;

    public Money Debit(Money currentValue, Money debit) => currentValue -= debit;
}