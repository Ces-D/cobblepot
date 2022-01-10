namespace Cobblepot.Domain.Accounting.Accounts.AccountTypes;

public interface IAccountValueCalculator
{
    AccountType AccountType { get; }
    Money Credit(Money currentValue, Money credit);
    Money Debit(Money currentValue, Money debit);
}